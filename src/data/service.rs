use std::fs;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::PathBuf;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::data::task::Task;
use crate::lib::bx::Bx;

#[derive(Serialize, Deserialize)]
pub enum Status {
    Prepare,
    Start,
    Stop,
}
#[derive(Serialize, Deserialize)]
pub struct Service{
    name: String,
    status: Status,
    time: DateTime<Local>,
    task: String,
}

impl Service{

    pub fn new_from_pathbuf_with_task_name(path: &PathBuf, task: &String) -> Service {
        let name = Bx::extract_filename_from_pathbuf(&path).unwrap(); // Den Dateinamen extrahieren.
        let status = Status::Stop; // Den Status auf "Stop" setzen.
        let time = Local::now(); // Die aktuelle lokale Zeit abrufen.

        // Hier kannst du alle anderen erforderlichen Initialisierungen für deinen Service vornehmen.

        // Zum Schluss den Service erstellen und zurückgeben.
        Service {
            name,
            status,
            time,
            task: task.clone(), // Du musst `task` klonen, wenn du den Besitz beibehalten möchtest.
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn time_to_string(&self) -> String {
        self.time.to_string()
    }

       pub fn start(task: &Task) {
           let mut path = PathBuf::new();

           if task.get_static_service() {
               println!("task ist static");
               path = Config::get_service_static_path();
           } else {
               println!("task ist temp");
               path = Config::get_service_temp_path();
           }

           //check ob schon ein service da ist

           if let Some(p) = find_next_prepare_or_stop_service(&task, &path){
               path = p;
           } else {
               println!("{} neue task prepared", Config::get_prefix());
               task.prepared_to_services();
               //path = find_next_prepare_or_stop_service(&task, &path).unwrap();
           }
           //start
            println!("start the service from task {}", task.get_name());


       }
}

fn find_next_prepare_or_stop_service(task: &Task, path: &PathBuf) -> Option<PathBuf> {
    let mut prefix = task.get_name();
    let _ = prefix.as_str();
    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.starts_with(&prefix) {
                        // Hier hast du einen Ordner mit dem gewünschten Präfix
                        // Jetzt kannst du die Nummer extrahieren und weiter verarbeiten
                        if let Some(mut rest) = file_name.strip_prefix(&prefix) {

                            if rest.starts_with('-') {
                                rest = &rest[1..]; // Das erste Zeichen abschneiden
                            }
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
                                    let mut service_path = path_service.clone();
                                    service_path.pop();
                                    service_path.pop();
                                    let service = Service::new_from_pathbuf_with_task_name(&service_path, &task.get_name());
println!("dwdwd");
                                    let default_config_str = serde_json::to_string_pretty(&service).expect("Fehler beim Serialisieren der Standardkonfiguration");

                                    let mut file = File::create(&path_service).expect("Fehler beim Erstellen der service_config.json");
                                    file.write_all(default_config_str.as_bytes()).expect("Fehler beim Schreiben in die service_config.json");
                                }

                                let file_content = read_to_string(&path_service).expect("Fehler beim Lesen von service_config.json");
                                let file: serde_json::Value = serde_json::from_str(&file_content).expect("Fehler beim Deserialisieren von service_config.json");

                                if let Some(status) = file["status"].as_str() {
                                    if !(status == "stop" || status == "prepare") {
                                        break;
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
