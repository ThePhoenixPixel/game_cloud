use std::fs;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::{Path, PathBuf};
use serde_json::json;
use crate::config::Config;
use crate::data::task::Task;
use crate::lib::bx::Bx;

enum Status {
    Start,
    Prepare,
    Stop,
}

pub struct Service{
    name: String,
    status: Status,
    max_players: u32,
    max_ram: u32,
    task: Task,
}

impl Service{
   pub fn start(task: &Task) {
       let mut path = PathBuf::new();

       if task.get_static_service() {
           println!("task ist static");
           path = Config::get_service_static_path();
       } else {
           println!("task ist temp");
           path = Config::get_service_temp_path();
       }
       if let Some(path) = find_next_prepare_or_stop_service(&task.get_name(), &path){

            println!("Ja wol");



        } else {
            println!("nööö")
        }

   }
}

fn find_next_prepare_or_stop_service(prefix: &str, path: &PathBuf) -> Option<PathBuf> {
    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries {
            if let Ok(entry) = entry {
                //
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.starts_with(prefix) {
                        // Hier hast du einen Ordner mit dem gewünschten Präfix
                        // Jetzt kannst du die Nummer extrahieren und weiter verarbeiten
                        if let Some(rest) = file_name.strip_prefix(prefix) {
                            if let Some(number) = rest.chars().next().and_then(|c| c.to_digit(10)) {
                                // Jetzt hast du die Nummer und kannst sie verwenden
                                println!("Found folder with prefix '{}', number: {}", prefix, number);

                                let mut path_service = path.clone();
                                path_service.push(format!("{}-{}", prefix, number));
                                path_service.push(".game_cloud");

                                if !path_service.exists() {
                                    // Wenn der Ordner nicht existiert, erstelle ihn
                                    fs::create_dir_all(&path_service).expect("Fehler beim Erstellen des .game_cloud Ordners");
                                }

                                path_service.push("service_config.json");

                                if !path_service.exists() {
                                    // Wenn die Datei nicht existiert, erstelle sie
                                    let default_config = json!({
                                        "status": "stop",
                                        // Weitere Standardwerte hier hinzufügen
                                    });

                                    let default_config_str = serde_json::to_string_pretty(&default_config).expect("Fehler beim Serialisieren der Standardkonfiguration");

                                    let mut file = File::create(&path_service).expect("Fehler beim Erstellen der service_config.json");
                                    file.write_all(default_config_str.as_bytes()).expect("Fehler beim Schreiben in die service_config.json");
                                }

                                let file_content = read_to_string(&path_service).expect("Fehler beim Lesen von service_config.json");
                                let file: serde_json::Value = serde_json::from_str(&file_content).expect("Fehler beim Deserialisieren von service_config.json");

                                if let Some(status) = file["status"].as_str() {
                                    if !(status == "stop" || status == "prepare") {
                                        break;
                                    }
                                }

                                return Some(path.clone());
                            }
                        }
                    }
                }
            }
        }
    }
    None
}
