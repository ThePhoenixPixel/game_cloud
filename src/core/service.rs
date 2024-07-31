use std::fs::{read_to_string, File};
use std::{fs, io};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use bx::Bx;
use bx::address::Address;
use bx::url::{Url, UrlSchema};

use crate::core::task::Task;
use crate::sys_config::cloud_config::CloudConfig;
use crate::sys_config::software_config::{SoftwareConfig, SoftwareName};
use crate::utils::logger::Logger;
use crate::utils::service_status::ServiceStatus;
use crate::{log_error, log_info};
use crate::core::network::requests::register_server::RegisterServerData;
use crate::core::software::Software;

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
    pub fn new(task: &Task) -> Result<Service, String> {
        let server_address = Address::new(
            &CloudConfig::get().get_server_host(),
            &Address::find_next_port(&mut Address::new(
                &CloudConfig::get().get_server_host(),
                &task.get_start_port(),
            )),
        );
        let service_path = match task.prepared_to_service() {
            Ok(path) => path,
            Err(e) => {
                return Err(format!("Es kann kein neuer Service erstellt werden \n {}", e))
            }
        };

        let service = Service {
            name: Bx::get_last_folder_name(&service_path),
            status: ServiceStatus::Stop,
            start_time: Local::now(),
            server_address,
            plugin_listener: Address::get_local(),
            cloud_listener: CloudConfig::get().get_node_host(),
            task: task.clone(),
        };

        service.save_to_file();
        return Ok(service);
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
        let address = self.find_free_server_address();

        let software = self.get_task().get_software();
        let software_type = match SoftwareConfig::get().get_software_type(&software.get_software_type()) {
            Some(software_type) => software_type,
            None => return Err(io::Error::new(io::ErrorKind::Other, "Can not get the Software Type")),
        };

        let software_name = match software_type.get_software_name(software.get_name().as_str()) {
            Some(software_name) => software_name,
            None => return Err(io::Error::new(io::ErrorKind::Other, "Can not get the Software Name")),
        };
        let path = self.get_path();

        // replace ip
        let path_ip = path.join(software_name.get_ip_path());
        let file_content_ip = read_to_string(&path_ip)?;
        let edit_file_ip = file_content_ip.replace("%ip%", &*address.get_ip());
        fs::write(&path_ip, edit_file_ip)?;

        // replace port
        let path_port = path.join(software_name.get_port_path());
        let file_content_port = read_to_string(&path_port)?;
        let edit_file_port = file_content_port.replace("%port%", address.get_port().to_string().as_str());
        fs::write(&path_port, edit_file_port)?;

        self.server_address = address;

        Ok(())
    }

    pub fn find_free_server_address(&self) -> Address {
        let ports = Service::get_bind_ports();
        let mut port = self.get_task().get_start_port();
        let server_host = CloudConfig::get().get_server_host();

        while ports.contains(&port) || !Address::is_port_available(&Address::new(&server_host, &port)) {
            port = Address::find_next_port(&mut Address::new(&server_host, &(port + 1)));
        }

        Address::new(&server_host, &port)
    }

    pub fn find_free_plugin_address(&self) -> Address {
        let ports = Service::get_bind_ports();
        let mut port = self.get_server_address().get_port() + 1;
        let server_host = CloudConfig::get().get_server_host();

        while ports.contains(&port) || !Address::is_port_available(&Address::new(&server_host, &port)) {
            port = Address::find_next_port(&mut Address::new(&server_host, &(port + 1)));
        }

        Address::new(&server_host, &port)
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
        let address = self.find_free_plugin_address();
        self.set_plugin_listener(&address);
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
    pub fn is_prepare(&self) -> bool {
        self.get_status().is_prepare()
    }
    pub fn is_stop(&self) -> bool {
        self.get_status().is_stop()
    }

    pub async fn reload() {
        let task_all = Task::get_task_all();

        for task in task_all {
            log_info!("Task: {} | Service: {}", task.get_name(), Service::get_starts_service_from_task(&task));

            let min_service_count = task.get_min_service_count() as u64;

            if min_service_count == 0 {
                continue;
            }

            reload_start(min_service_count, &task).await;
        }
    }

    pub fn get_starts_service_from_task(task: &Task) -> u64 {
        let service_path = task.get_service_path();
        let mut start_service: u64 = 0;
        let files_name = Bx::get_files_name_from_path(&service_path);

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

    pub fn is_service_start(path: &mut PathBuf) -> bool {
        return match Service::get_from_path(path) {
            Some(service) => service.is_start(),
            None => false,
        };
    }

    pub async fn start(&mut self) -> Result<(), io::Error> {
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
        Ok(())
    }

    fn prepare_to_start(&mut self) -> Result<(), io::Error> {
        self.install_software()?;
        self.install_system_plugin()?;
        self.set_server_address()?;
        self.find_new_free_plugin_listener();
        self.set_status(&ServiceStatus::Prepare);
        Ok(())
    }

    pub fn get_service_url(&self) -> Url {
        Url::new(UrlSchema::Http, &self.get_plugin_listener(), "cloud/service").join(&self.get_name())
    }

    pub async fn connect_to_proxy(&self) -> Result<(), String> {
        if self.is_proxy() {
            return Err("The Service is a Proxy".to_string());
        }

        let service_proxy_list = Service::get_online_proxy_server();

        for service_proxy in service_proxy_list {
            println!("url -> {}", service_proxy.get_service_url().join("registerService").get());
            match service_proxy.get_service_url().join("registerService").post(&RegisterServerData::create_request(&self, &true)).await {
                Ok(_) => log_info!("erfolgreich connect to ...."),
                Err(e) => log_info!("{}", e.to_string()),
            }
        }
        Ok(())
    }

    pub fn get_online_proxy_server() -> Vec<Service> {
        let services = Service::get_online_service();
        let mut proxy_server_list: Vec<Service> = Vec::new();

        for service in services {
            if service.is_proxy() {
                proxy_server_list.push(service)
            }
        }
        proxy_server_list
    }

    pub fn get_online_backend_server() -> Vec<Service> {
        let services = Service::get_online_service();
        let mut backend_server_list: Vec<Service> = Vec::new();

        for service in services {
            if !service.is_proxy() {
                backend_server_list.push(service)
            }
        }
        backend_server_list
    }

    pub fn get_all_service() -> Vec<Service> {
        let mut service_list: Vec<Service> = Vec::new();
        let service_path_dir = CloudConfig::get().get_cloud_path().get_service_folder().get_temp_folder_path();
        for folder in Bx::get_folders_name_from_path(&service_path_dir) {
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
    pub fn get_prepare_service() -> Vec<Service> {
        let mut service_prepare_list: Vec<Service> = Vec::new();
        let service_list = Service::get_all_service();
        for service in service_list {
            if service.is_prepare() {
                service_prepare_list.push(service);
            }
        }
        service_prepare_list
    }

    pub fn get_offline_service() -> Vec<Service> {
        let mut service_offline_list: Vec<Service> = Vec::new();
        let service_list = Service::get_all_service();
        for service in service_list {
            if service.is_stop() {
                service_offline_list.push(service);
            }
        }
        service_offline_list
    }

    pub fn get_from_name(name: &String) -> Option<Service> {
        let mut path = CloudConfig::get().get_cloud_path().get_service_folder().get_temp_folder_path().join(&name);
        return Service::get_from_path(&mut path);
    }

    pub fn install_software(&self) -> Result<(), io::Error> {
        let target_path = self.get_path().join(&self.get_software().get_name_with_ext());
        let software_path = self.get_software().get_software_file_path();

        fs::copy(&software_path, &target_path)?;
        Ok(())
    }

    pub fn install_system_plugin(&self) -> Result<(), io::Error> {
        let software = match self.get_software().get_software_from_software_config() {
            Some(software) => software,
            None => return Err(io::Error::other("Software wurde nicht gefunden um das System Plugin zu installieren")),
        };
        let system_plugin_path = self.get_task().get_software().get_system_plugin_path();
        let mut target_path = self.get_path().join(&software.get_system_plugin().get_path());

        if !target_path.exists() {
            Bx::create_path(&target_path);
        }

        target_path.push(self.get_software().get_system_plugin_name());

        return match fs::copy(
            system_plugin_path,
            target_path,
        ) {
            Ok(_) => {
                log_info!("Successfully install the System Plugin");
                Ok(())
            }
            Err(e) => return Err(e),
        };
    }

    pub fn get_software(&self) -> Software {
        self.get_task().get_software()
    }

    pub fn is_proxy(&self) -> bool {
        self.get_software().get_software_type().to_lowercase() == "proxy"
    }

    pub fn get_next_stop_service(task: &Task) -> Result<Service, io::Error> {
        let offline_services = Service::get_offline_service();
        for offline_service in offline_services {
            if !(offline_service.get_task() == task.clone()) {
                continue;
            }
            return Ok(offline_service);
        }

        return match Service::new(&task) {
            Ok(service) => Ok(service),
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
        };
    }

    pub fn get_bind_ports() -> Vec<u32> {
        let mut ports = Service::get_bind_server_ports();
        ports.append(&mut Service::get_bind_plugin_ports());
        ports
    }

    pub fn get_bind_server_ports() -> Vec<u32> {
        let mut services = Service::get_online_service();
        services.append(&mut Service::get_prepare_service());
        let mut ports: Vec<u32> = Vec::new();

        for service in services {
            ports.push(service.get_server_address().get_port())
        }
        ports
    }

    pub fn get_bind_plugin_ports() -> Vec<u32> {
        let mut services = Service::get_online_service();
        services.append(&mut Service::get_prepare_service());
        let mut ports: Vec<u32> = Vec::new();

        for service in services {
            ports.push(service.get_plugin_listener().get_port())
        }
        ports
    }

    pub async fn shutdown(&self) {
        //match self.get_service_url().join("shutdown").post()
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

async fn reload_start(min_service_count: u64, task: &Task) {
    for _ in 0..min_service_count {
        if !(min_service_count > Service::get_starts_service_from_task(&task))
        {
            continue;
        }
        log_info!(
            "Service would be create from task: {}",
            task.get_name()
        );
        log_info!("---------------------------------------------------------------");
        let mut service = match Service::get_next_stop_service(&task) {
            Ok(service) => service,
            Err(e) => {
                log_error!("{}", e);
                return;
            }
        };
        match service.start().await {
            Ok(_) => log_info!("Server [{}] successfully start :=)", service.get_name()),
            Err(e) => log_error!("Server [{}] can NOT Start \n {}", service.get_name(), e),
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
