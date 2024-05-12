use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::{fs, io};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use reqwest::Client;
use serde_json::json;

use crate::core::task::Task;
use crate::lib::address::Address;
use crate::lib::bx::Bx;
use crate::sys_config::cloud_config::CloudConfig;
use crate::sys_config::software_config::{SoftwareConfig, SoftwareName};
use crate::utils::logger::Logger;
use crate::utils::path::Path;
use crate::utils::service_status::ServiceStatus;
use crate::{log_error, log_info, log_warning};

#[derive(Serialize, Deserialize)]
pub struct Service {
    name: String,
    status: ServiceStatus,
    start_time: DateTime<Local>,
    server_address: Address,
    plugin_listener: Address,
    cloud_listener: Address,
    task: Task,
}

impl Service {
    pub fn new(task: &Task) -> Service {
        let server_address = Address::new(
            &CloudConfig::get().get_server_host(),
            &Address::find_next_port(&mut Address::new(
                &CloudConfig::get().get_server_host(),
                &task.get_start_port(),
            )),
        );

        let plugin_listener = Address::new(
            &"127.0.0.1".to_string(),
            &mut Address::find_next_port(&mut Address::new(
                &"127.0.0.1".to_string(),
                &mut task.get_start_port(),
            )),
        );
        let cloud_listener = CloudConfig::get().get_node_host();

        let service = Service {
            name: format!(
                "{}-{}",
                task.get_name(),
                Service::get_next_free_number(&task)
            ),
            status: ServiceStatus::Stop,
            start_time: Local::now(),
            server_address,
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

    pub fn get_server_address(&self) -> Address {
        self.server_address.clone()
    }

    pub fn set_server_address(&mut self) -> Result<(), io::Error> {
        let address = Address::new(
            &CloudConfig::get().get_server_host(),
            &Address::find_next_port(&mut Address::new(
                &CloudConfig::get().get_server_host(),
                &self.get_task().get_start_port(),
            )),
        );
        let software = self.get_task().get_software();
        let software_type = match SoftwareConfig::get().get_software_type(software.get_software_type()) {
            Some(software_type) => software_type,
            None => return Err(io::Error::new(io::ErrorKind::Other, "Can not get the Software Type")),
        };

        let software_name = match software_type.get_software_name(software.get_name().as_str()) {
            Some(software_name) => software_name,
            None => return Err(io::Error::new(io::ErrorKind::Other, "Can not get the Software Name")),
        };
        let path = self.get_path();

        // replace ip
        let path_ip = path.join(software_name.get_ip().get_path());
        let file_content_ip = read_to_string(&path_ip)?;
        let edit_file_ip = file_content_ip.replace("%ip%", &*address.get_ip());
        fs::write(&path_ip, edit_file_ip)?;

        // replace port
        let path_port = path.join(software_name.get_port().get_path());
        let file_content_port = read_to_string(&path_port)?;
        let edit_file_port = file_content_port.replace("%port%", address.get_port().to_string().as_str());
        fs::write(&path_port, edit_file_port)?;

        self.server_address = address;

        Ok(())
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
        let mut address = Address::new(&self.get_plugin_listener().get_ip(), &(self.get_server_address().get_port() + 1));
        if Address::is_port_available(&address) {
            let _ = &self.set_plugin_listener(&address);
        } else {
            self.set_plugin_listener(&Address::new(&address.get_ip(), &Address::find_next_port(&mut address)));
        }
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
            log_error!("Error by create to service config file");
            return;
        }

        if let Ok(serialized) = serde_json::to_string_pretty(&self) {
            if let Ok(mut file) = File::create(self.get_path_with_service_file()) {
                file.write_all(serialized.as_bytes())
                    .expect("Error by save the service config file");
            }
        }
    }

    pub fn is_start(&self) -> bool {
        self.get_status().is_start()
    }

    pub fn reload() {
        let task_all = Task::get_task_all();

        for task in task_all {
            log_info!("Task: {} | Service: {}", task.get_name(), Service::get_starts_service_from_task(&task));

            let min_service_count = task.get_min_service_count() as u64;

            if min_service_count == 0 {
                continue;
            }

            reload_start(min_service_count, &task);
        }
    }

    pub fn get_starts_service_from_task(task: &Task) -> u64 {
        let service_path = task.get_service_path();
        let mut start_service: u64 = 0;
        let files_name = Path::get_files_name_from_path(&service_path);

        for file_name in files_name {
            let mut current_service_path = service_path.clone();
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

    pub fn start(&mut self) -> Result<(), io::Error> {
        self.prepare_to_start()?;

        let stdout_file = File::create(self.get_path_stdout_file())?;
        let _stdin_file = File::create(self.get_path_stdin_file())?;
        let stderr_file = File::create(self.get_path_stderr_file())?;

        let software = match self
            .get_task()
            .get_software()
            .get_software_from_software_config()
        {
            Some(software) => software,
            None => {
                return Err(io::Error::new(io::ErrorKind::Other,
                                          format!("Can not find the Software for the service {}", self.get_name())));
            }
        };

        let server_file_path = match self.get_path_server_file().to_str() {
            Some(server_file_path) => server_file_path.to_string(),
            None => return Err(io::Error::new(io::ErrorKind::Other, "Can not server file path to string change")),
        };

        let server_path = match self.get_path().to_str() {
            Some(server_file_path) => server_file_path.to_string(),
            None => return Err(io::Error::new(io::ErrorKind::Other, "Can not server path to string change")),
        };

        let stdin_file = File::open("/dev/null")?;

        // data copy
        let software_clone = software.clone();
        let stdout_file_clone = stdout_file.try_clone()?;
        let stderr_file_clone = stderr_file.try_clone()?;
        let stdin_file_clone = stdin_file.try_clone()?;
        let max_ram = self.get_task().get_max_ram();
        // ThreadManager create und Thread start

        start_server(
            software_clone,
            server_file_path,
            max_ram,
            stdout_file_clone,
            stderr_file_clone,
            stdin_file_clone,
            server_path,
        );

        match self.connect_to_proxy() {
            Ok(_) => log_info!("Service [{}] connect to Proxy", &self.get_name()),
            Err(e) => log_warning!("{}", e),
        }
        Ok(())
    }

    pub fn connect_to_proxy(&self) -> Result<(), String> {
        if self.get_task().get_software().get_software_type() == "proxy" {
            return Err("The Service is a Proxy".to_string());
        }

        let url = "http://127.0.0.1:25566/service/Proxy-1/registerService".to_string();
        let client = Client::new();
        let mut json_body = json!(
            {
              "registerServer": {
                "name": "lobby",
                "ip": "127.0.0.1",
                "port": 30068,
                "try_to_connect": true
              }
            }
        );

        if let Some(register_server) = json_body.get_mut("registerServer") {
            if let Some(name) = register_server.get_mut("name") {
                *name = json!(self.get_name());
            }
            if let Some(ip) = register_server.get_mut("ip") {
                *ip = json!(self.get_server_address().get_ip());
            }
            if let Some(port) = register_server.get_mut("port") {
                *port = json!(self.get_server_address().get_port());
            }
        }
        println!("[Debug] Service add to connect to proxy -> {:?}", json_body);

        let _ = client.post(url.to_string())
            .json(&json_body)
            .send();
        Ok(())
    }

    fn prepare_to_start(&mut self) -> Result<(), io::Error> {
        // set the ports
        self.set_server_address()?;
        self.find_new_free_plugin_listener();
        Ok(())
    }

    pub fn get_all_service() -> Vec<Service> {
        let mut service_list: Vec<Service> = Vec::new();
        let service_path_dir = CloudConfig::get().get_cloud_path().get_service_folder().get_temp_folder_path();
        for folder in Path::get_folders_name_from_path(&service_path_dir) {
            let mut path = service_path_dir.clone();
            path.push(folder);
            if let Some(service) = Service::get_from_path(&mut path) {
                service_list.push(service);
            };
        }
        service_list
    }

    pub fn get_online_service() -> Vec<Service> {
        let mut service_online_list: Vec<Service> = Vec::new();
        let service_list = Service::get_all_service();
        for service in service_list {
            if service.is_start() {
                service_online_list.push(service);
            }
        }
        service_online_list
    }
}

fn start_server<'a>(
    software: SoftwareName,
    server_file_path: String,
    max_ram: u32,
    stdout_file: File,
    stderr_file: File,
    stdin_file: File,
    service_path: String,
) {
    let _server = Command::new(software.get_command())
        .args(&[
            format!("-Xmx{}M", max_ram),
            "-jar".to_string(),
            server_file_path,
        ])
        .arg("nogui")
        .current_dir(service_path)
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file))
        .stdin(Stdio::from(stdin_file))
        .spawn()
        .expect("Error by start the Server");
}

fn reload_start(min_service_count: u64, task: &Task) {
    for _ in 0..min_service_count {
        if !(min_service_count > Service::get_starts_service_from_task(&task))
        {
            continue;
        }
        println!(
            "Service would be create from task: {}",
            task.get_name()
        );
        let mut service = Service::new(&task);
        match service.start() {
            Ok(_) => log_info!("Server [{}] successfully start :=)", service.get_name()),
            Err(e) => log_error!("Server [{}] cant start \n {}", service.get_name(), e),
        }
    }
}