use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

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
        let file_content = match fs::read_to_string(
            &CloudConfig::get()
                .get_cloud_path()
                .get_system_folder()
                .get_software_config_path(),
        ) {
            Ok(file_content) => file_content,
            Err(e) => {
                log_warning!("Bitte gebe den richtigen Pfad zur Software-Dateikonfiguration an");
                log_error!("{}", &e.to_string());
                get_default_file()
            }
        };

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

    fn new(software_type: HashMap<String, SoftwareType>) -> SoftwareConfig {
        SoftwareConfig { software_type }
    }

    pub fn get_software_type(&self, software_type: &str) -> Option<SoftwareType> {
        self.software_type.get(software_type).cloned()
    }

    pub fn get_software_types(&self) -> HashMap<String, SoftwareType> {
        self.software_type.clone()
    }

    pub fn add_software_type(&mut self, name: &String, software_type: &SoftwareType) {
        self.software_type
            .insert(name.to_string(), software_type.clone());
    }

    pub fn remove_software_type(&mut self, name: &str) {
        self.software_type.remove(name);
    }

    pub fn check() {
        if !CloudConfig::get()
            .get_cloud_path()
            .get_system_folder()
            .get_software_config_path()
            .exists()
        {
            SoftwareConfig::install();
        }
    }

    pub fn install() {
        let url = "https://download.codergames.de/game_cloud/v0.1/config/software.json";
        let mut folder_path = CloudConfig::get()
            .get_cloud_path()
            .get_system_folder()
            .get_software_config_path();

        folder_path.pop();
        match Bx::download_file(url, &folder_path) {
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
    fn new(software_name: Vec<SoftwareName>) -> SoftwareType {
        SoftwareType { software_name }
    }

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

    pub fn add_software_name(&mut self, software_name: &SoftwareName) {
        self.software_name.push(software_name.clone());
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
    command: String,
    max_ram: u32,
    ip: IP,
    port: Port,
}

impl SoftwareName {
    fn new(
        name: &str,
        download: &str,
        command: &str,
        ram: &u32,
        ip: &IP,
        port: &Port,
    ) -> SoftwareName {
        SoftwareName {
            name: name.to_string(),
            download: download.to_string(),
            command: command.to_string(),
            max_ram: ram.clone(),
            ip: ip.clone(),
            port: port.clone(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: &String) {
        self.name = name.clone();
    }

    pub fn get_download(&self) -> String {
        self.download.clone()
    }

    pub fn set_download(&mut self, download: &String) {
        self.download = download.clone();
    }

    pub fn get_command(&self) -> String {
        self.command.clone()
    }

    pub fn set_command(&mut self, command: &String) {
        self.command = command.clone();
    }

    pub fn get_ip(&self) -> IP {
        self.ip.clone()
    }

    pub fn get_port(&self) -> Port {
        self.port.clone()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IP {
    path: String,
    content: String,
}

impl IP {
    pub fn new(path: &str, content: &str) -> IP {
        IP {
            path: path.to_string(),
            content: content.to_string(),
        }
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
    }

    pub fn set_path(&mut self, path: &str) {
        self.path = path.to_string()
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_string()
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Port {
    path: String,
    content: String,
}

impl Port {
    pub fn new(path: &str, content: &str) -> Port {
        Port {
            path: path.to_string(),
            content: content.to_string(),
        }
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
    }

    pub fn set_path(&mut self, path: &str) {
        self.path = path.to_string()
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_string()
    }
}

fn save_to_file(
    config: &SoftwareConfig,
    file_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // Serialize SoftwareConfig to a JSON string
    let json_str = serde_json::to_string_pretty(config)?;

    // Open or create the file for writing
    let mut file = File::create(file_path)?;

    // Write the JSON string to the file
    file.write_all(json_str.as_bytes())?;

    Ok(())
}

fn get_default_file() -> String {
    let json_str = r#"
    {
      "software_type": {
        "server": {
          "software_name": [
            {
              "name": "paper",
              "download": "https://paper.de",
              "command": "java",
              "ip": {
                "path": "server.propeties",
                "content": "server-ip:%ip%"
              },
              "port": {
                "path": "server.propeties",
                "content": "server-port:%port%"
              }
            }
          ]
        }
      }
    }
    "#;
    json_str.to_string()
}
