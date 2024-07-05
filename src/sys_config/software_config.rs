use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use crate::lib::bx::Bx;
use crate::sys_config::cloud_config::CloudConfig;
use crate::utils::logger::Logger;
use crate::{log_error, log_info, log_warning};


#[derive(Serialize, Deserialize, Clone)]
pub struct SoftwareConfig {
    software_type: HashMap<String, SoftwareType>,
}

impl SoftwareConfig {
    pub fn get() -> SoftwareConfig {
        let file_content = fs::read_to_string(
            &CloudConfig::get()
                .get_cloud_path()
                .get_system_folder()
                .get_software_config_path(),
        ).unwrap_or_else(|e| {
            log_warning!("Bitte gebe den richtigen Pfad zur Software-Dateikonfiguration an");
            log_error!("{}", &e.to_string());
            get_default_file()
        });

        // Versuche, den Inhalt der Datei zu deserialisieren
        return match serde_json::from_str(&file_content) {
            Ok(config) => config,
            Err(e) => {
                log_warning!("Fehler beim Deserialisieren der Software-Dateikonfiguration");
                log_error!("{}", &e.to_string());
                panic!("The GameCloud has an fatal Error");
            }
        };
    }

    pub fn get_software_type(&self, software_type: &str) -> Option<SoftwareType> {
        self.software_type.get(software_type).cloned()
    }

    pub fn get_software_types(&self) -> HashMap<String, SoftwareType> {
        self.software_type.clone()
    }


    pub fn remove_software_type(&mut self, name: &str) {
        self.software_type.remove(name);
    }

    pub fn check(url: &String) {
        if !CloudConfig::get()
            .get_cloud_path()
            .get_system_folder()
            .get_software_config_path()
            .exists()
        {
            SoftwareConfig::install(url);
        }
    }

    pub fn install(start_url: &String) {
        let url = format!("{}/config/software.json", start_url);
        let mut folder_path = CloudConfig::get()
            .get_cloud_path()
            .get_system_folder()
            .get_software_config_path()
            .join("software.json");

        folder_path.pop();
        match Bx::download_file(url.as_str(), &folder_path) {
            Ok(_) => log_info!("Successfully download the Software Config from {}", url),
            Err(e) => {
                log_error!("{}", e);
                panic!("Game Cloud has an fatal Error");
            }
        }
    }
}

// -----------------------------------------------------------
#[derive(Serialize, Deserialize, Clone)]
pub struct SoftwareType {
    software_name: Vec<SoftwareName>,
}

impl SoftwareType {
    pub fn get_software_name(&self, name: &str) -> Option<SoftwareName> {
        for software in &self.software_name {
            if software.get_name() == name {
                return Some(software.clone());
            }
        }
        None
    }

    pub fn get_software_names(&self) -> Vec<SoftwareName> {
        self.software_name.clone()
    }

    pub fn remove_software_name(&mut self, software_name: &SoftwareName) {
        self.software_name
            .insert(self.software_name.len() + 1, software_name.clone());
    }
}

//-------------------------------------------------------------
#[derive(Serialize, Deserialize, Clone)]
pub struct SoftwareName {
    name: String,
    download: String,
    system_plugin: SystemPlugin,
    command: String,
    max_ram: u32,
    ip_path: String,
    port_path: String,
}

impl SoftwareName {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_download(&self) -> String {
        self.download.clone()
    }

    pub fn get_command(&self) -> String {
        self.command.clone()
    }

    pub fn get_ip_path(&self) -> String {
        self.ip_path.clone()
    }

    pub fn get_port_path(&self) -> String {
        self.port_path.clone()
    }

    pub fn get_system_plugin(&self) -> SystemPlugin {
        return self.system_plugin.clone();
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub struct SystemPlugin {
    local: bool,
    download: String,
    path: String,
}

impl SystemPlugin {
    pub fn is_local(&self) -> bool {
        return self.local;
    }
    pub fn get_download(&self) -> String {
        return self.download.clone();
    }
    pub fn get_path(&self) -> String {
        return self.path.clone();
    }
}

fn get_default_file() -> String {
    let json_str = r#"
{
  "software_type": {
    "server": {
      "software_name": [
        {
          "name": "paper",
          "download": "https://api.papermc.io/v2/projects/paper/versions/1.20.4/builds/389/downloads/paper-1.20.4-389.jar",
          "system_plugin": {
            "local": false,
            "download": "http://download.codergames.de/minecloud/version/0.1/config/system_plugins/MineCloud-Paper.jar",
            "path": "plugins/"
          },
          "command": "java",
          "max_ram": 1024,
          "ip_path": "server.properties",
          "port_path": "server.properties"
        }
      ]
    },
    "proxy": {
      "software_name": [
        {
          "name": "velocity",
          "download": "https://api.papermc.io/v2/projects/velocity/versions/3.3.0-SNAPSHOT/builds/323/downloads/velocity-3.3.0-SNAPSHOT-323.jar",
          "system_plugin": {
            "local": false,
            "download": "http://download.codergames.de/minecloud/version/0.1/config/system_plugins/MineCloud-Velocity.jar",
            "path": "plugins/"
          },
          "command": "java",
          "max_ram": 512,
          "ip_path": "velocity.toml",
          "port_path": "velocity.toml"
        },
        {
          "name": "waterfall",
          "download": "https://api.papermc.io/v2/projects/waterfall/versions/1.20/builds/562/downloads/waterfall-1.20-562.jar",
          "system_plugin": {
            "local": false,
            "download": "http://download.codergames.de/minecloud/version/0.1/config/system_plugins/MineCloud-Waterfall.jar",
            "path": "plugins/"
          },
          "command": "java",
          "max_ram": 512,
          "ip_path": "config.yml",
          "port_path": "config.yml"
        }
      ]
    }
  }
}
    "#;
    json_str.to_string()
}
