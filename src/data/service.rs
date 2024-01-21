use crate::config::Config;
use crate::data::task::Task;
use crate::lib::address::Address;
use crate::lib::bx::Bx;
use crate::lib::thread_manager::ThreadManager;
use crate::logger::Logger;
use crate::sys_config::software_config::{SoftwareConfig, SoftwareName};
use crate::utils::path::Path;
use crate::utils::service_status::ServiceStatus;
use crate::Main;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Serialize, Deserialize)]
pub struct Service {
    name: String,
    status: ServiceStatus,
    start_time: DateTime<Local>,
    plugin_listener: Address,
    cloud_listener: Address,
    task: Task,
}

impl Service {
    // es werden neue oder prepard service zurückgegeben da die fn get_next_free_number() den nächsten nicht start service zurückgibt
    pub fn new(task: &Task) -> Service {
        let plugin_listener = Address::new(
            &"127.0.0.1".to_string(),
            &Address::find_next_port(&Address::new(
                &"127.0.0.1".to_string(),
                &task.get_start_port(),
            )),
        );
        let cloud_listener = Config::get_node_listener();

        let service = Service {
            name: format!(
                "{}-{}",
                task.get_name(),
                Service::get_next_free_number(&task)
            ),
            status: ServiceStatus::Stop,
            start_time: Local::now(),
            plugin_listener,
            cloud_listener,
            task: task.clone(),
        };
        service.get_task().prepared_to_services();
        service.save_to_file();
        service
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: &String) {
        self.name = name.clone();
        self.save_to_file();
    }

    pub fn get_status(&self) -> ServiceStatus {
        self.status.clone()
    }

    pub fn set_status(&mut self, status: &ServiceStatus) {
        self.status = status.clone();
        self.save_to_file();
    }

    pub fn get_time_to_string(&self) -> String {
        self.start_time.to_string()
    }

    pub fn set_time(&mut self) {
        self.start_time = Local::now();
        self.save_to_file();
    }

    pub fn get_task(&self) -> Task {
        self.task.clone()
    }

    pub fn get_plugin_listener(&self) -> Address {
        self.plugin_listener.clone()
    }

    pub fn set_plugin_listener(&mut self, address: &Address) {
        self.plugin_listener = address.clone();
        self.save_to_file();
    }

    pub fn get_cloud_listener(&self) -> Address {
        self.cloud_listener.clone()
    }

    pub fn set_cloud_listener(&mut self, address: Address) {
        self.cloud_listener = address;
        self.save_to_file();
    }

    pub fn set_server_address(&self) {
        let address = Address::new(
            &Config::get_server_host(),
            &Address::find_next_port(&Address::new(
                &Config::get_server_host(),
                &self.get_task().get_start_port(),
            )),
        );
        let software = self.get_task().get_software();
        let software_type =
            match SoftwareConfig::get().get_software_type(software.get_software_type()) {
                Some(software_type) => software_type,
                None => {
                    Logger::error("Can not get the Software Type");
                    return;
                }
            };

        let software_name = match software_type.get_software_name(software.get_name().as_str()) {
            Some(software_name) => software_name,
            None => {
                Logger::error("Can not get the Software Name");
                return;
            }
        };
        let mut path = self.get_path();
        path.push(software_name.get_ip().get_path());

        let file_to_string = fs::read_to_string(&path).expect("Error ganz blöd");
        let file_content: Value =
            serde_json::from_str(&file_to_string).expect("Error der blöd ist");
    }

    pub fn get_path(&self) -> PathBuf {
        let mut path = self.get_task().get_service_path();
        path.push(self.get_name());
        path
    }

    pub fn get_path_server_file(&self) -> PathBuf {
        self.get_path()
            .join(self.get_task().get_software().get_name_with_ext())
    }

    pub fn get_path_stdout_file(&self) -> PathBuf {
        let mut path = self.get_path_with_service_file();
        path.pop();
        path.push("server_stdout.log");
        path
    }

    pub fn get_path_stdin_file(&self) -> PathBuf {
        let mut path = self.get_path_with_service_file();
        path.pop();
        path.push("server_stdin.log");
        path
    }

    pub fn get_path_stderr_file(&self) -> PathBuf {
        let mut path = self.get_path_with_service_file();
        path.pop();
        path.push("server_stderr.log");
        path
    }

    pub fn find_new_free_plugin_listener(&mut self) {
        if Address::is_port_available(&self.get_plugin_listener()) {
            return;
        }
        self.set_plugin_listener(&Address::new(
            &"127.0.0.1".to_string(),
            &Address::find_next_port(&Address::new(
                &"127.0.0.1".to_string(),
                &self.get_task().get_start_port(),
            )),
        ));
        self.save_to_file();
    }

    pub fn get_path_with_service_file(&self) -> PathBuf {
        let mut path = self.get_task().get_service_path();
        path.push(self.get_name());
        path.push(".game_cloud");
        path.push("service_config.json");
        path
    }

    pub fn get_from_path(path: &mut PathBuf) -> Option<Service> {
        //path -> /service/temp/Lobby-1/
        path.push(".game_cloud");
        path.push("service_config.json");
        if let Ok(file_content) = read_to_string(path) {
            if let Ok(service) = serde_json::from_str(&file_content) {
                Some(service)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn save_to_file(&self) {
        let mut path = self.get_path_with_service_file();
        path.pop();
        Bx::create_path(&path);
        if File::create(self.get_path_with_service_file()).is_err() {
            Logger::error("Fehler beim erstellen der service config datei");
            return;
        }

        if let Ok(serialized) = serde_json::to_string_pretty(&self) {
            if let Ok(mut file) = File::create(self.get_path_with_service_file()) {
                file.write_all(serialized.as_bytes())
                    .expect("Error beim save to file der service_config datei");
            }
        }
    }

    pub fn is_start(&self) -> bool {
        self.get_status().is_start()
    }

    pub fn reload() {
        let task_all = Task::get_task_all();

        for task in task_all {
            println!(
                "{} Task: {} Gestartete Service: {}",
                Config::get_prefix(),
                task.get_name(),
                Service::get_starts_service_from_task(&task)
            );

            if !(task.get_min_service_count() > 0) {
                break;
            }

            for _ in 0..task.get_min_service_count() {
                if !(task.get_min_service_count() as u64
                    > Service::get_starts_service_from_task(&task))
                {
                    break;
                }
                println!(
                    "{} Service wird gestartet von task: {}",
                    Config::get_prefix(),
                    task.get_name()
                );
                let mut service = Service::new(&task);
                service.start();
            }
        }
    }

    pub fn get_starts_service_from_task(task: &Task) -> u64 {
        let service_path = task.get_service_path();
        let mut start_service: u64 = 0;
        let files_name = Path::get_files_name_from_path(&service_path);

        for file_name in files_name {
            let mut current_service_path = service_path.clone(); // Kopiere den service_path für diesen Schleifendurchlauf
            if file_name.starts_with(&task.get_name()) {
                current_service_path.push(file_name);

                if Service::is_service_start(&mut current_service_path) {
                    start_service += 1;
                }
            }
        }
        start_service
    }

    pub fn get_next_free_number(task: &Task) -> u64 {
        let mut next_number_to_check: u64 = 1;
        loop {
            let service_name = format!("{}-{}", task.get_name(), next_number_to_check);
            let mut path = task.get_service_path();

            path.push(service_name);
            path.push(".game_cloud");
            path.push("service_config.json");

            let service = match Service::get_from_path(&mut path) {
                Some(service) => service,
                None => return next_number_to_check,
            };

            // Check service ist started
            if !service.is_start() {
                return next_number_to_check;
            }

            next_number_to_check += 1;
        }
    }

    pub fn is_service_start(path: &mut PathBuf) -> bool {
        return match Service::get_from_path(path) {
            Some(service) => service.is_start(),
            None => false,
        };
    }

    pub fn start(&mut self) {
        println!("in der start fn");
        self.prepare_to_start();
        println!("nach prepare to start");

        let stdout_file = match File::create(self.get_path_stdout_file()) {
            Ok(file) => file,
            Err(e) => {
                Logger::error(e.to_string().as_str());
                return;
            }
        };

        let stdin_file = match File::create(self.get_path_stdin_file()) {
            Ok(file) => file,
            Err(e) => {
                Logger::error(e.to_string().as_str());
                return;
            }
        };

        let stderr_file = match File::create(self.get_path_stderr_file()) {
            Ok(file) => file,
            Err(e) => {
                Logger::error(e.to_string().as_str());
                return;
            }
        };

        let software = match self
            .get_task()
            .get_software()
            .get_software_from_software_config()
        {
            Some(software) => software,
            None => {
                Logger::error(
                    format!(
                        "Can not find the Software for the service {}",
                        self.get_name()
                    )
                    .as_str(),
                );
                return;
            }
        };
        let server_file_path = match self.get_path_server_file().to_str() {
            Some(server_file_path) => server_file_path,
            None => {
                Logger::error("Can not server file path to string change");
                return;
            }
        }
        .to_string();

        let server_path = match self.get_path().to_str() {
            Some(server_file_path) => server_file_path,
            None => {
                Logger::error("Can not server path to string change");
                return;
            }
        }
        .to_string();

        let stdin_file = File::open("/dev/null").expect("Fehler beim Öffnen der Standardeingabe");

        println!("{}", server_file_path);
        println!("{}", software.get_command());

        let mut thread_manager = ThreadManager::new();

        // Daten kopieren
        let software_clone = software.clone();
        let stdout_file_clone = stdout_file
            .try_clone()
            .expect("Failed to clone stdout file");
        let stderr_file_clone = stderr_file
            .try_clone()
            .expect("Failed to clone stderr file");
        let stdin_file_clone = stdin_file.try_clone().expect("Failed to clone stdin file");
        let max_ram = self.get_task().get_max_ram();
        // ThreadManager erstellen und Thread starten
        thread_manager.spawn(move || {
            start_server(
                software_clone,
                server_file_path,
                max_ram,
                stdout_file_clone,
                stderr_file_clone,
                stdin_file_clone,
                server_path,
            );
        });

        println!("start the server");

        thread_manager.shutdown_all();

        //self.connect_to_proxy();
    }

    pub fn connect_to_proxy(&self) {}

    fn prepare_to_start(&mut self) {
        println!("in prepare to start");
        // the ports set
        self.set_server_address();

        self.find_new_free_plugin_listener();
        println!("nach check service sys_config");
    }
}

fn start_server<'a>(
    software: SoftwareName, // Software als Kopie übergeben
    server_file_path: String,
    max_ram: u32,
    stdout_file: File,
    stderr_file: File,
    stdin_file: File,
    service_path: String, // Service-Pfad ebenfalls übergeben
) {
    let server = Command::new(software.get_command())
        .args(&[
            format!("-Xmx{}M", max_ram),
            "-jar".to_string(),
            server_file_path,
        ])
        .current_dir(service_path)
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file))
        .stdin(Stdio::from(stdin_file))
        .spawn()
        .expect("Fehler beim Starten des Servers");
}
