use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
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
       let prefix = format!("{}-", task.get_name());




   }
}

fn find_next_prepare_or_stop_service(prefix: &String, path: &PathBuf) -> Option<PathBuf> {
    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.starts_with(&prefix) {
                        // Hier hast du einen Ordner mit dem gewünschten Präfix
                        // Jetzt kannst du die Nummer extrahieren und weiter verarbeiten
                        if let Some(rest) = file_name.strip_prefix(&prefix) {
                            if let Some(number) = rest.chars().next() {
                                if let Ok(parsed_number) = number.to_digit(10) {
                                    // Jetzt hast du die Nummer und kannst sie verwenden
                                    println!("Found folder with prefix '{}', number: {}", prefix, parsed_number);
                                    let _ = path.join(format!("{}-{}", prefix, parsed_number));

                                    let mut path_service = path.clone();
                                    path_service.push(".game_cloud");

                                    if !path_service.exists() {
                                        //println!("Service: {:?} ist ungültig", path);
                                        break;
                                    }

                                    path_service.push("service_config.json");
                                    if !path_service.exists() {
                                        break
                                    }




                                    return Some(path.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}