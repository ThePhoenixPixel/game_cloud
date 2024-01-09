use std::{env, fs};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use serde::Serialize;
use serde_json::Value;
use crate::config::Config;

#[derive(Serialize, Clone)]
pub struct Software{
    pub software_type: String,
    pub name: String,
}

impl Software {

    pub fn new() -> Software {
        Software{
            software_type: "Server".to_string(),
            name: "paper".to_string(),
        }
    }
    //get Software


    //software type
    pub fn get_software_type(&self) -> &String {
        &self.software_type
    }

    pub fn set_software_type(&mut self, software_type: &String) {
        self.software_type = software_type.clone();
    }

    //name
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_name_with_ext(&self) -> String {
        let name = &self.get_name();
        let binding = &self.get_software_url().unwrap();
        let link = Path::new(&binding);
        return if let Some(ext) = link.extension().and_then(|ext| ext.to_str()) {
            format!("{}.{}", name, ext)
        } else {
            // Fallback, wenn keine Dateiendung gefunden wurde
            format!("{}", name)
        }
    }


    pub fn set_name(&mut self, name: &String) {
        self.name = name.clone();
    }

    pub fn get_host_path(&self) -> Option<PathBuf> {
        let mut path = env::current_exe().expect("Error beim getten des exe path");
        let config = Software::get_config();
        match config["software"][&self.get_software_type()][&self.get_name()]["host"]["path"].as_str() {
            Some(content) => {
                Some(path.join(content))
            }
            None => None
        }
    }

    pub fn get_port_path(&self) -> Option<PathBuf> {
        let mut path = env::current_exe().expect("Error beim getten des exe path");
        let config = Software::get_config();
        match config["software"][&self.get_software_type()][&self.get_name()]["port"]["path"].as_str() {
            Some(content) => {
                Some(path.join(content))
            }
            None => None
        }
    }

    pub fn get_host_content(&self) -> Option<String> {
        let config = Software::get_config();

        match config["software"][&self.get_software_type()][&self.get_name()]["host"]["content"].as_str() {
            Some(content) => Some(content.to_string()),
            None => None,
        }
    }


    pub fn get_port_content(&self) -> Option<String> {
        let config = Software::get_config();
        match config["software"][&self.get_software_type()][&self.get_name()]["port"]["content"] {
            Some(content) => content,
            None => None,
        }
    }

    pub fn set_host(&self) -> bool {
        let software_config = Software::get_config();
        let software_content = match self.get_host_content() {
            Some(content) => content,
            None => return false,
        };
        

        return true;
    }


    pub fn get_software_url(&self) -> Option<String> {
        let software_path = Config::get_software_path();

        let config_content = fs::read_to_string(&software_path).expect("Fehler beim Lesen der Software Datei");
        let config: Value = serde_json::from_str(&config_content).expect("Fehler beim Deserialisieren der Konfiguration");

        if let Some(software) = config.get("software") {
            for (_, software_type_value) in software.as_object().unwrap() {
                if let Some(download_url) = software_type_value.get(self.get_name().as_str()) {
                    if let Some(url) = download_url.as_str() {
                        return Some(url.to_string());
                    }
                }
            }
        }

        None // Software nicht gefunden
    }

    pub fn get_config() -> Value {
        let software_path = Config::get_software_path();

        let config_content = fs::read_to_string(&software_path).expect("Fehler beim Lesen der Software Datei");
        let config = serde_json::from_str(&config_content).expect("Fehler beim Deserialisieren der Konfiguration");
        config
    }

    pub fn get_software_file_path(&self) -> PathBuf {
        let mut software_path = Config::get_software_files_path();
        software_path.push(&self.get_software_type());
        software_path.push((format!("{}.jar", &self.get_name())));
        software_path
    }

}