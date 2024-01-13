use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::PathBuf;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::data::task::Task;
use crate::lib::address::Address;
use crate::utils::path::Path;
use crate::utils::service_status::ServiceStatus;

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
        let plugin_listener = Address::new(&"127.0.0.1".to_string(), &Address::find_next_port(&"127.0.0.1".to_string(), task.get_start_port()));
        let cloud_listener = Config::get_node_listener();

        Service {
            name: format!("{}-{}", task.get_name(), Service::get_next_free_number(&task)),
            status: ServiceStatus::Stop,
            start_time: Local::now(),
            plugin_listener,
            cloud_listener,
            task: task.clone(),
        }
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

    pub fn get_path(&self) -> PathBuf {
        let mut path = self.get_task().get_service_path();
        path.push(self.get_task().get_name());
        path
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
        if let Ok(serialized) = serde_json::to_string_pretty(&self) {
            if let Ok(mut file) = File::create(self.get_path_with_service_file()) {
                file.write_all(serialized.as_bytes()).expect("Error beim save to file der service_config datei");
            }
        }
    }

    pub fn is_start(&self) -> bool {
        self.get_status().is_start()
    }

    pub fn reload() {
        let task_all = Task::get_task_all();

        for task in task_all {
            println!("{} Task: {} Gestartete Service: {}", Config::get_prefix(), task.get_name(), Service::get_starts_service_from_task(&task));

            if !(task.get_min_service_count() > 0) {
                break;
            }

            for _ in 0..task.get_min_service_count() {
                if !(task.get_min_service_count() as u64 > Service::get_starts_service_from_task(&task)) {
                    break;
                }
                println!("{} Service wird gestartet von task: {}", Config::get_prefix(), task.get_name());
                let service = Service::new(&task);
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
            let mut path = if task.get_static_service() {
                Config::get_service_static_path()
            } else {
                Config::get_service_temp_path()
            };

            path.push(service_name);
            path.push(".game_cloud");
            path.push("service_config.json");

            let service = match Service::get_from_path(&mut path) {
                Some(service) => service,
                None => return next_number_to_check
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
        }
    }

    pub fn start(&self) {
        println!("in der start fn");
    }

    pub fn connect_to_proxy(&self) {

    }

}


/*

fn extract_and_install_links(software_type: &str, software_links: &Map<String, Value>){
    //let install_dir = Config::get_software_files_path();
    let cmd_prefix = Config::get_prefix();

    // Iteriere durch die Kategorien (self, server, proxy)
    // Iteriere durch die Software-Links in diesem Software-Typ (Kategorie)
    for (software_name, software_link_value) in software_links.iter() {
        if let Some(software_link) = software_link_value.as_str() {
            //let software_dir = install_dir.join(software_type);
            install_software(software_link, software_name, software_type, &cmd_prefix);

        } else {
            println!(
                "{} Ungültiger Link für Software: {}",
                cmd_prefix, software_name
            );
        }
    }
*/