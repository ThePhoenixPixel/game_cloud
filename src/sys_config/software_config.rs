use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SofwareConfig{
    software_type: HashMap<String, SoftwareType>
}

impl SofwareConfig {
    fn new(software_type: HashMap<String, SoftwareType>) -> SofwareConfig {
        SofwareConfig {
            software_type
        }
    }
}
// -----------------------------------------------------------
#[derive(Serialize, Deserialize)]
pub struct SoftwareType {
    software_name: Vec<SoftwareName>
}

impl SoftwareType {
    fn new(software_name: Vec<SoftwareName>) -> SoftwareType {
        SoftwareType {
            software_name
        }
    }
}
//-------------------------------------------------------------
#[derive(Serialize, Deserialize)]
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
}


fn main() {
    // Beispiel SofwareConfig erstellen
    let software_name_1 = SoftwareName::new("http://dw1", "java");
    let software_name_2 = SoftwareName::new("http://dw2", "python");

    let mut software_type_1_map = HashMap::new();
    software_type_1_map.insert("paper".to_string(), software_name_1);

    let mut software_type_2_map = HashMap::new();
    software_type_2_map.insert("velocity".to_string(), software_name_2);

    let mut software_config_map = HashMap::new();
    software_config_map.insert("server".to_string(), SoftwareType::new(software_type_1_map));

    let software_config = SofwareConfig::new(software_config_map);

    // Speichern in eine Datei
    if let Err(err) = save_to_file(&software_config, &PathBuf::from("config.json")) {
        eprintln!("Fehler beim Speichern der Konfiguration");
    } else {
        println!("Konfiguration erfolgreich in 'config.json' gespeichert.");
    }

}

fn save_to_file(config: &SofwareConfig, file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Serialize SoftwareConfig to a JSON string
    let json_str = serde_json::to_string_pretty(config)?;

    // Open or create the file for writing
    let mut file = File::create(file_path)?;

    // Write the JSON string to the file
    file.write_all(json_str.as_bytes())?;

    Ok(())
}