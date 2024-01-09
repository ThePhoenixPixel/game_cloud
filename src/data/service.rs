use chrono::{DateTime, Local};

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
    pub fn new(task: &Task) -> Service {
        let temp_port:Option<u32> = Config::get_node_port().try_into().ok();
        let mut port:u32 = 0;
        match temp_port {
            Some(result) => {
                port = result;
            }
            None => {
                println!("umwandlung nicht möglich");
            }
        }

        let plugin_listener = Address::new(&"127.0.0.1".to_string(), &Address::find_next_port(&"127.0.0.1".to_string(), task.get_start_port()));
        let cloud_listener = Address::new(&Config::get_node_host(), &port);

        Service {
            name: service_name.clone(),
            status: ServiceStatus::Stop,
            start_time: Local::now(),
            plugin_listener,
            cloud_listener,
            task,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: &String) {
        self.name = name.clone();
        self.save_to_file();
    }

    pub fn get_status(&self) -> String {
        self.status.clone();
    }

    pub fn set_status(&mut self, status: ServiceStatus) {
        self.status = status.clone();
        self.save_to_file();
    }

    pub fn get_time_to_string(&self) -> String {
        self.start_time.to_string
    }

    pub fn set_time(&mut self) {
        self.start_time = Local::now();
        self.save_to_file();
    }

    pub fn get_task(&self) -> Task {
        match self.task {
            Ok(task) => task,
        }
    }

    pub fn get_plugin_listener(&self) -> Address {
        self.plugin_listener
    }

    pub fn set_plugin_listener(&mut self, address: &Address) {
        self.plugin_listener = address;
        self.save_to_file();
    }

    pub fn get_cloud_listener(&self) -> Address {
        self.cloud_listener
    }

    pub fn set_cloud_listener(&mut self, address: Address) {
        self.cloud_listener = address;
        self.save_to_file();
    }

    pub fn get_path(&self) -> PathBuf {
        let mut path = PathBuf::new();
        if self.get_task().get_static_service() {
            path = Config::get_service_static_path()
        } else {
            path = Config::get_service_temp_path()
        }
        path.push(task.get_name())
    }

    pub fn get_path_with_service_file(&self) -> PathBuf {
        let mut path = PathBuf::new();
        if self.get_task().get_static_service() {
            path = Config::get_service_static_path()
        } else {
            path = Config::get_service_temp_path()
        }
        path.push(task.get_name());
        path.push(".game_cloud");
        path.push("service_config.json")
    }

     pub fn load_from_file(service_name: &String, is_static: bool) -> Option<Service> {
        let mut file_path = if is_static {
            Config::get_service_static_path
        } else {
            Config::get_service_temp_path
        };
        file_path.push(service_name);
        
        if let Ok(file_content) = read_to_string(file_path) {
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

        for task_name in task_all {
            if let Some(task) = Task::get_task(task_name) {
                println!("{} Gestartete Service: {}",Config::get_prefix(), Service::get_start_service_from_task(&task));
                if task.get_min_service_count() > 0 {
                    for _ in 0..task.get_min_service_count() {
                        println!("Dienst starten from {}", &task.get_name());
                        if task.get_min_service_count() > Service::get_start_service_from_task(&task) as u32 {
                            println!("gestartete service: {}", Service::get_start_service_from_task(&task) as u32);
                            println!("Start the the service from task {} ", task.get_name());
                            Service::start(&task);
                        }
                    }
                } else {
                    println!("{} Task: {} muss kein service gestartet werden", Config::get_prefix(), task.get_name());
                }
            } else {
                println!("{} task error", Config::get_prefix());
            }
        }
    }




}


