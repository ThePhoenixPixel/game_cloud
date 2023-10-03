use std::fs;
use std::path::PathBuf;
use serde::Serialize;
use serde_json::Value;
use crate::config::Config;

#[derive(Serialize, Clone)]
pub struct Software{
    pub software_type: String,
    pub name: String,
    pub max_ram: u32,
}
impl Software{
    pub fn new() -> Software{
        Software{
            software_type: "Server".to_string(),
            name: "paper".to_string(),
            max_ram: 1024,
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

    pub fn set_name(&mut self, name: &String) {
        self.name = name.clone();
    }

    //max ram
    pub fn get_max_ram(&self) -> &u32 {
        &self.max_ram
    }

    pub fn set_max_ram(&mut self, max_ram: &u32) {
        self.max_ram = max_ram.clone();
    }

    pub fn get_software_url(&self) -> Option<String> {
        let software_path = Config::get_software_path();

        let config_content = fs::read_to_string(&software_path).expect("Fehler beim Lesen der Software Datei");
        let config: Value = serde_json::from_str(&config_content).expect("Fehler beim Deserialisieren der Konfiguration");


        let software_map = match config.get(&self.get_software_type()) {
            Some(Value::Object(map)) => map,
            _ => return None, // Ungültiger Typ
        };

        if let Some(download_url) = software_map.get(self.get_name().as_str()) {
            if let Some(url) = download_url.as_str() {
                return Some(url.to_string());
            }
        }


        None // Software nicht gefunden
    }

    pub fn get_software_file_path(&self) -> PathBuf {
        let mut software_path = Config::get_software_files_path();
        software_path.push(&self.get_software_type());
        software_path.push(&self.get_name());
        software_path
    }

}