use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct SoftwareConfig {
    software_type: HashMap<String, SoftwareType>,
}

impl SoftwareConfig {
    fn new(software_type: HashMap<String, SoftwareType>) -> SoftwareConfig {
        SoftwareConfig { software_type }
    }

    pub fn get_software_type(&self) -> HashMap<String, SoftwareType> {
        self.software_type.clone()
    }

    pub fn add_software_type(&mut self, name: &String, software_type: &SoftwareType) {
        self.software_type
            .insert(name.to_string(), software_type.clone());
    }

    pub fn remove_software_type(&mut self, name: &str) {
        self.software_type.remove(name);
    }

    pub fn test() {
        let mut software_name = Vec::new();
        software_name.push(SoftwareName::new("paper", "http://paper.de", "java"));

        let software_type_1 = SoftwareType::new(software_name);
        let mut software_type = HashMap::new();
        software_type.insert("server".to_string(), software_type_1);

        let software_config = SoftwareConfig::new(software_type);

        save_to_file(&software_config, &PathBuf::from("software_config.json"))
            .expect("Error sace to file");
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

    pub fn get_software_name(&self) -> Vec<SoftwareName> {
        self.software_name.clone()
    }

    pub fn add_software_name(&mut self, software_name: &SoftwareName) {
        self.software_name.push(software_name.clone());
    }

    pub fn remove_software_name(&mut self, software_name: &SoftwareName) {
        self.software_name
            .insert((self.software_name.len() + 1), software_name.clone());
    }
}
//-------------------------------------------------------------
#[derive(Serialize, Deserialize, Clone)]
pub struct SoftwareName {
    name: String,
    download: String,
    command: String,
}

impl SoftwareName {
    fn new(name: &str, download: &str, command: &str) -> SoftwareName {
        SoftwareName {
            name: name.to_string(),
            download: download.to_string(),
            command: command.to_string(),
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
