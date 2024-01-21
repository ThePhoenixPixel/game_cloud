use crate::lib::address::Address;
use crate::Main;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct CloudConfig {
    name: String,
    prefix: String,
    language: String,
    server_host: String,
    max_ram: u64,
    node_host: Address,
    rest_api: Address,
    path: CloudConfigPath,
}

impl CloudConfig {
    pub fn get() -> CloudConfig {
        let path = Main::get_exe_path().join("config.json");
        // Versuche, den Inhalt der Datei zu lesen
        let file_content = match fs::read_to_string(&path) {
            Ok(file_content) => file_content,
            Err(e) => {
                eprintln!("{}", &e.to_string());
                get_default_file()
            }
        };

        // Versuche, den Inhalt der Datei zu deserialisieren
        return match serde_json::from_str(&file_content) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Fehler beim Deserialisieren der Config");
                eprintln!("{}", &e.to_string());
                panic!("The GameCloud has an fatal Error");
            }
        };
    }

    pub fn new(
        name: String,
        prefix: String,
        language: String,
        server_host: String,
        max_ram: u64,
        node_host: Address,
        rest_api: Address,
        path: CloudConfigPath,
    ) -> CloudConfig {
        CloudConfig {
            name,
            prefix,
            language,
            server_host,
            max_ram,
            node_host,
            rest_api,
            path,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_prefix(&self) -> &String {
        &self.prefix
    }

    pub fn get_language(&self) -> &String {
        &self.language
    }

    pub fn get_server_host(&self) -> &String {
        &self.server_host
    }

    pub fn get_max_ram(&self) -> u64 {
        self.max_ram
    }

    pub fn get_node_host(&self) -> &Address {
        &self.node_host
    }

    pub fn get_rest_api(&self) -> &Address {
        &self.rest_api
    }

    pub fn get_path(&self) -> &CloudConfigPath {
        &self.path
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CloudConfigPath {
    task_folder: String,
    template_folder: String,
    service_folder: CloudConfigService,
    system_folder: CloudConfigSystem,
}

impl CloudConfigPath {
    pub fn new(
        task_folder: String,
        template_folder: String,
        service_folder: CloudConfigService,
        system_folder: CloudConfigSystem,
    ) -> CloudConfigPath {
        CloudConfigPath {
            task_folder,
            template_folder,
            service_folder,
            system_folder,
        }
    }

    pub fn get_task_folder(&self) -> &String {
        &self.task_folder
    }

    pub fn get_template_folder(&self) -> &String {
        &self.template_folder
    }

    pub fn get_service_folder(&self) -> &CloudConfigService {
        &self.service_folder
    }

    pub fn get_system_folder(&self) -> &CloudConfigSystem {
        &self.system_folder
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CloudConfigService {
    temp_folder: String,
    static_folder: String,
}

impl CloudConfigService {
    pub fn new(temp_folder: String, static_folder: String) -> CloudConfigService {
        CloudConfigService {
            temp_folder,
            static_folder,
        }
    }

    pub fn get_temp_folder(&self) -> &String {
        &self.temp_folder
    }

    pub fn get_static_folder(&self) -> &String {
        &self.static_folder
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CloudConfigSystem {
    software_config: String,
    default_task: String,
    system_plugins_folder: String,
    software_files_folder: String,
}

impl CloudConfigSystem {
    pub fn new(
        software_config: String,
        default_task: String,
        system_plugins_folder: String,
        software_files_folder: String,
    ) -> CloudConfigSystem {
        CloudConfigSystem {
            software_config,
            default_task,
            system_plugins_folder,
            software_files_folder,
        }
    }

    pub fn get_software_config(&self) -> &String {
        &self.software_config
    }

    pub fn get_default_task(&self) -> &String {
        &self.default_task
    }

    pub fn get_system_plugins_folder(&self) -> &String {
        &self.system_plugins_folder
    }

    pub fn get_software_files_folder(&self) -> &String {
        &self.software_files_folder
    }
}

fn get_default_file() -> String {
    let json_str = r#"
    {
      "name": "Node-1",
      "prefix": "[Game Cloud]",
      "language": "de",
      "server_host": "127.0.0.1",
      "max_ram": 2028,
      "node_host": {
        "ip": "127.0.0.1",
        "port": 5005
      },
      "rest_api": {
        "ip": "127.0.0.1",
        "port": 6006
      },
      "path": {
        "task": "task",
        "template": "template",
        "service": {
          "temp": "service/temp",
          "static": "service/static"
        },
        "config": {
          "software": "config",
          "default_task": "config",
          "system_plugins": "config/system_plugins",
          "software_files": "config/software_files"
        }
      }
    }
    "#;
    json_str.to_string()
}
