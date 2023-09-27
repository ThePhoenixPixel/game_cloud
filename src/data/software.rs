use std::fs;
use serde::Serialize;
use serde_json::Value;
use crate::config::Config;

#[derive(Serialize)]
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

    pub fn set_software_type(&mut self, software_type: String) {
        self.software_type = software_type;
    }

    //name
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    //max ram
    pub fn get_max_ram(&self) -> &u32 {
        &self.max_ram
    }

    pub fn set_max_ram(&mut self, max_ram: u32) {
        self.max_ram = max_ram;
    }

    pub fn get_software_url(software_type: &str, software_name: &str) -> Option<String> {
        let software_path = Config::get_software_path();

        let config_content = fs::read_to_string(&software_path).expect("Fehler beim Lesen der Software Datei");
        let config: serde_json::Value = serde_json::from_str(&config_content).expect("Fehler beim Deserialisieren der Konfiguration");


        let software_map = match config.get(software_type) {
            Some(Value::Object(map)) => map,
            _ => return None, // Ungültiger Typ
        };

        if let Some(download_url) = software_map.get(software_name) {
            if let Some(url) = download_url.as_str() {
                return Some(url.to_string());
            }
        }

        None // Software nicht gefunden
    }

}