use std::fs;
use std::fs::{read_to_string};
use std::path::PathBuf;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::data::task::Task;
use crate::lib::bx::Bx;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Status {
    Prepare,
    Start,
    Stop,
}
#[derive(Serialize, Deserialize)]
pub struct Service{
    name: String,
    status: String,
    time: DateTime<Local>,
    listener: String,
    task: String,
}

impl Service {
    pub fn new_from_pathbuf_with_task_name(path: &PathBuf, task: &String) -> Service {
        let name = Bx::extract_filename_from_pathbuf(&path).unwrap(); // Den Dateinamen extrahieren.
        let status = String::from("Stop"); // Den Status auf "Stop" setzen.
        let listener = Config::get_node_listener();
        let time = Local::now(); // Die aktuelle lokale Zeit abrufen.

        // Hier kannst du alle anderen erforderlichen Initialisierungen für deinen Service vornehmen.

        // Zum Schluss den Service erstellen und zurückgeben.
        Service {
            name,
            status,
            time,
            listener,
            task: task.clone(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_status(&self) -> &String {
        &self.status
    }

    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }

    pub fn get_time_to_string(&self) -> String {
        self.time.to_string()
    }

    pub fn get_start_service_from_task(task: &Task) -> u64 {
        let mut service_path = PathBuf::new();

        let start_service:u64 = 0;

        if task.get_static_service() {
            service_path = Config::get_service_static_path();
        } else {
            service_path = Config::get_service_temp_path();
        }

        let files_name = get_files_name_from_path(&service_path);

        for file_name in files_name {
            if file_name.starts_with(&task.get_name()) {
                service_path.push(file_name);
                if let Some(start) = is_service_start(&mut service_path) {
                    if start {
                        start_service + 1;
                    }
                } else {
                    println!("{} Error beim service {:?}", Config::get_prefix(), &service_path);
                }
            }
        }
        start_service
    }

    pub fn start(task: &Task) {
        println!("in der start fn ----------------");
        let mut path = PathBuf::new();

        if task.get_static_service() {
            println!("task ist static");
            path = Config::get_service_static_path();
        } else {
            println!("task ist temp");
            path = Config::get_service_temp_path();
        }
        let split = '-';
        let next_number = get_next_free_service_with_start(&path, &task.get_name(), &split);
        println!("{}", next_number);



    }
}


fn get_next_free_service_with_start(path: &PathBuf, name: &String, split: &char) -> u64 {
    // Lies den Inhalt des Verzeichnisses
    let dir_contents = fs::read_dir(path).expect("Kann Verzeichnis nicht lesen");

    // Erstelle einen Vektor, um die vorhandenen Nummern zu speichern
    let mut existing_numbers: Vec<u64> = vec![];

    for entry in dir_contents {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.starts_with(name) && file_name.contains(*split) {
                    // Extrahiere die Nummer aus dem Dateinamen
                    let parts: Vec<&str> = file_name.split(*split).collect();

                    if parts.len() > 1 {
                        if let Ok(number) = parts[1].parse::<u64>() {
                            // Die Zahl wurde erfolgreich extrahiert und kann nun als u64 verwendet werden
                            existing_numbers.push(number);

                            let mut service_path = entry.path();
                            println!("{:?}", service_path);
                            if let Some(start) = is_service_start(&mut service_path) {
                                if !start {
                                    return number; // Die Nummer ist verfügbar
                                }
                            }
                        } else {
                            println!("Fehler: Konnte die Zahl nicht extrahieren.");
                        }
                    } else {
                        println!("Fehler: Ungültiges Format");
                    }
                }
            }
        }
    }

    // Finde die nächste verfügbare Nummer
    let mut next_number: u64 = 1;
    while existing_numbers.contains(&next_number) {
        next_number += 1;
    }

    next_number
}

fn is_service_start(path: &mut PathBuf) -> Option<bool> {
    path.push(".game_cloud");
    path.push("service_config.json");
    println!("{:?}", path);
    let file_content = read_to_string(&path).expect("Fehler beim Lesen von service_config.json");
    let file: serde_json::Value = serde_json::from_str(&file_content).expect("Fehler beim Deserialisieren von service_config.json");

    if let Some(status) = file["status"].as_str() {
        println!("in startus");
        return if status == "Stop" || status == "Prepare" {
            Some(false)
        } else {
            Some(true)
        }
    }
    None
}

fn get_files_name_from_path(path: &PathBuf) -> Vec<String> {
    let mut files_name: Vec<String> = Vec::new();
    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    files_name.push(file_name.to_string());
                }
            }
        }
    }
    files_name
}


/*
fn check_service(path: &mut PathBuf, task_name: &String) {
    //path darf nur  /service/temp/service-1 oder /service/static/service-1
    create_service_folder(path);
    create_service_file(path, task_name);

    path.push(task_name);
    if let Some(boolean) = is_service_start(path) {
        if boolean {
            println!("start");
        } else {
            println!("create new");
        }
    } else {
        println!("Fehler haft");
        println!("Lösche den alten .game_cloud ordner");
    }
}



fn create_service_file(path: &mut PathBuf, task_name: &String) {
    path.push("service_config.json");
    println!("efefefefef");
    println!("{:?}", path);
    if !path.exists() { // Wenn die Datei nicht existiert, erstelle sie
        println!("fefwdf3r3r3r");
        let mut service_path = path.clone();
        service_path.pop();
        service_path.pop();
        let service = Service::new_from_pathbuf_with_task_name(&service_path, &task_name);

        let default_config_str = serde_json::to_string_pretty(&service).expect("Fehler beim Serialisieren der Standardkonfiguration");
        println!("{:?}", path);
        let mut file = File::create(&path).expect("Fehler beim Erstellen der service_config.json");
        file.write_all(default_config_str.as_bytes()).expect("Fehler beim Schreiben in die service_config.json");
    }
}

fn create_service_folder(path: &mut PathBuf) {
    path.push(".game_cloud");
    //create the .game_cloud folder in service
    if !path.exists() {
        Bx::create_path(&path);
    }
}

fn get_file_name_number(file_name: &mut String, split: &char) -> Option<u32> {
    if let Some(index) = file_name.find(*split) {
        file_name.drain(0..index + 1);
    }

    if let Some(number) = file_name.chars().next().and_then(|c| c.to_digit(10)) {
        Some(number)
    } else {
        println!("{} Keine Number", Config::get_prefix()); // Ersetze Config::get_prefix() entsprechend.
        None
    }
}

    fn find_next_prepare_or_stop_service(task: &Task, path: &PathBuf) -> Option<PathBuf> {
    let prefix = task.get_name();

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



                      //-------------------------------------------------------------------------------------
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

                                    let default_config_str = serde_json::to_string_pretty(&service).expect("Fehler beim Serialisieren der Standardkonfiguration");

                                    let mut file = File::create(&path_service).expect("Fehler beim Erstellen der service_config.json");
                                    file.write_all(default_config_str.as_bytes()).expect("Fehler beim Schreiben in die service_config.json");
                                }
                                println!("Found folder with prefix '{}', number: {}", prefix, number);
                                let file_content = read_to_string(&path_service).expect("Fehler beim Lesen von service_config.json");
                                let file: serde_json::Value = serde_json::from_str(&file_content).expect("Fehler beim Deserialisieren von service_config.json");

                                if let Some(status) = file["status"].as_str() {
                                    println!("in startus");
                                    if status == "stop" || status == "prepare" {
                                        return Some(path_service);
                                    }
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


fn prepare_or_start_service(service_path: &PathBuf, task_name: &String) -> Option<Service> {
    let file_content = read_to_string(&service_path).expect("Fehler beim Lesen von service_config.json");
    let file: serde_json::Value = serde_json::from_str(&file_content).expect("Fehler beim Deserialisieren von service_config.json");

    if let Some(status) = file["status"].as_str() {
        if status == "stop" || status == "prepare" {
            // Starte den Service, indem du ihn aus der Datei lädst
            let service = Service::new_from_pathbuf_with_task_name(&service_path, task_name);
            Some(service)
        } else {
            // Der Status ist nicht "stop" oder "prepare", also starte nicht.
            None
        }
    } else {
        // Status nicht gefunden, tue nichts.
        None
    }
}
*/