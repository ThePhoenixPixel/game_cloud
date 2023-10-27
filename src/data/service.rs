use std::fs;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Command;
use chrono::{DateTime, Local};
use chrono::format::parse;
use colored::Colorize;
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

    pub fn set_status(&mut self, status: &String) {
        self.status = status.clone();
        self.save_to_file();
    }

    pub fn get_task_name(&self) -> String{
        self.task.clone()
    }


    pub fn get_time_to_string(&self) -> String {
        self.time.to_string()
    }

    pub fn set_time(&mut self, time: &DateTime<Local>) {
        self.time = time.clone();
        self.save_to_file();
    }

    pub fn save_to_file(&self) {
        let serialized = serde_json::to_string_pretty(&self).unwrap();

        let task = Task::get_task(self.get_task_name()).expect("Error");
        // get der path entweder temp oder static
        let mut  path = PathBuf::new();
        if task.get_static_service() {
            path = Config::get_service_static_path();
        } else {
            path = Config::get_service_temp_path();
        }
        path.push(self.get_name());
        path.push(".game_cloud");
        path.push("service_config.json");

        if let Ok(mut file) = File::create(&path) {
            file.write_all(serialized.as_bytes()).expect("Fehler beim Schreiben der Datei");
        }
    }

    // Methode zum Laden des Service aus einer Datei
    pub fn load_from_file(file_path: &PathBuf) -> Option<Service> {
        if let Ok(file_content) = read_to_string(file_path) {
            if let Ok(service) = serde_json::from_str(&file_content) {
                Some(service)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn is_start(&self) -> bool {
        if self.status == "Start".to_string() {
            return true
        }
        return false;
    }
    pub fn reload() {
        let task_all = Task::get_task_all();

        for task_name in task_all {
            if let Some(task) = Task::get_task(task_name) {
                println!("{} Gestartete Service: {}",Config::get_prefix(), Service::get_start_service_from_task(&task));
                if task.get_min_service_count() > 0 {
                    for _ in 0..task.get_min_service_count() {
                        println!("Dienst starten from {}", &task.get_name());
                        if task.get_min_service_count() > Service::get_start_service_from_task(&task) as u32 {
                            println!("gestartete service: {}", Service::get_start_service_from_task(&task) as u32);
                            println!("Start the the service from task {} ", task.get_name());
                            Service::start(&task);
                        }
                    }
                } else {
                    println!("{} Task: {} muss kein service gestartet werden", Config::get_prefix(), task.get_name());
                }
            } else {
                println!("{} task error", Config::get_prefix());
            }
        }
    }

    pub fn get_start_service_from_task(task: &Task) -> u64 {
        let mut service_path = PathBuf::new();

        let mut start_service: u64 = 0;

        if task.get_static_service() {
            service_path = Config::get_service_static_path();
        } else {
            service_path = Config::get_service_temp_path();
        }

        let files_name = get_files_name_from_path(&service_path);

        for file_name in files_name {
            let mut current_service_path = service_path.clone(); // Kopiere den service_path für diesen Schleifendurchlauf
            if file_name.starts_with(&task.get_name()) {
                current_service_path.push(file_name);

                if is_service_start(&mut current_service_path) {
                    start_service += 1;
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
        let next_number = get_next_free_number(&path, &task.get_name()).expect("Error beim get next number");
        let service_name = format!("{}{}{}", task.get_name(), task.get_split(), next_number);
        path.push(&service_name);
        prepare_to_start(&mut path, &task);
        println!("{}", next_number);
        println!("{} Start tgferhgiwhgierhgrhguihgherghe {} Service", Config::get_prefix(), service_name);

        if let Some(mut service) = Service::load_from_file(&path) {
            path.pop();
            path.pop();

            println!("Service Name: {}", service.get_name());
            println!("Service Status: {}", service.get_status());
            println!("Service Time: {}", service.get_time_to_string());
            let mut status = String::from("Prepare");
            service.set_status(&status);
            println!("Service startet gerade");
            //service start

            start(&task, &service, &path);

            status = String::from("Start");
            service.set_status(&status);
            println!("{} Service ist gestartet {}", Config::get_prefix(), service.get_name());
        } else {
            println!("Eroor beim getten des service")
        }
    }
}

fn start(task: &Task, service: &Service, path: &PathBuf){
    if let Some(port) = find_next_port(task.get_start_port()) {




        let server = Command::new("java")
            .args(&[format!("-Xmx{}M", task.get_max_ram()),
                "-jar".to_string(),
                path.join(task.get_software().get_name_with_ext()).to_str().unwrap().to_string()])

            .current_dir(&path)
            .spawn()
            .expect("Fehler beim Starten des Servers");




    } else {
        println!("{} Service kann nicht starten from {}", Config::get_prefix(), task.get_name());
    }
}

fn find_next_port(start_port: u32) -> Option<u32> {
    let mut port = start_port;
    let max_port: u32 = 65535;
    while port <= max_port {
        if is_port_available(port) {
            return Some(port); // Verwende 'return' hier, um den gefundenen Port zurückzugeben
        }
        port += 1;
    }
    println!("{} Error es ist kein freier Port gefunden worden", Config::get_prefix());
    None
}

fn is_port_available(port: u32) -> bool {
    let host = Config::get_server_host(); // Lade die Server-Host-Adresse
    let socket_addr = format!("{}:{}", host, port);

    if let Ok(listener) = TcpListener::bind(&socket_addr) {
        // Port ist verfügbar
        drop(listener);
        true
    } else {
        // Port ist bereits in Verwendung
        false
    }
}

fn prepare_to_start(service_path: &mut PathBuf, task: &Task) {
    if !is_service_start(service_path) {
        if !service_path.exists() {
            task.prepared_to_services()
        }
        service_path.pop();
        service_path.pop();
        check_folder(service_path, &task.get_name());
    }
}


fn get_next_free_number(path: &PathBuf, prefix: &str) -> Option<u64> {
    let mut next_number_to_check: u64 = 1;
    loop {
        let service_name = format!("{}-{}", prefix, next_number_to_check);
        let mut full_path = path.join(&service_name);

        if !full_path.exists() {
            return Some(next_number_to_check);
        }

        // Überprüfe, ob der Service gestartet ist
        if !is_service_start(&mut full_path) {
            return Some(next_number_to_check);
        }

        next_number_to_check += 1;
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
                            //existing_numbers.push(number);

                            let mut service_path = entry.path();
                            println!("{:?}", service_path);

                            if !is_service_start(&mut service_path) {
                                return number; // Die Nummer ist verfügbar

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
fn is_service_start(path: &mut PathBuf) -> bool {
    //println!("{:?}", path);
    path.push(".game_cloud");
    path.push("service_config.json");
    //println!("{:?}", path);
    match read_to_string(&path) {
        Ok(file_content) => {
            let file: serde_json::Value = serde_json::from_str(&file_content).expect("Fehler beim Deserialisieren von service_config.json");
            if let Some(status) = file["status"].as_str() {
                return if status != "Stop" || status != "Prepare" {
                    true
                } else {
                    false
                }
            }
        }
        Err(_) => {
            // Fehler beim Lesen der Datei
            return false
        }
    }
    false
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

fn check_folder(path: &mut PathBuf, task_name: &String) {
    //println!("in check folder {:?}", path);
    path.push(".game_cloud");
    create_service_folder(path);
    create_service_file(path, task_name);
}

fn create_service_file(path: &mut PathBuf, task_name: &String) {
    path.push("service_config.json");
    if !path.exists() { // Wenn die Datei nicht existiert, erstelle sie
        let mut service_path = path.clone();
        service_path.pop();
        service_path.pop();
        let service = Service::new_from_pathbuf_with_task_name(&service_path, &task_name);

        let default_config_str = serde_json::to_string_pretty(&service).expect("Fehler beim Serialisieren der Standardkonfiguration");
        //println!("{:?}", path);
        let mut file = File::create(&path).expect("Fehler beim Erstellen der service_config.json");
        file.write_all(default_config_str.as_bytes()).expect("Fehler beim Schreiben in die service_config.json");
    }
}

fn create_service_folder(path: &mut PathBuf) {
    //create the .game_cloud folder in service
    //println!("{:?}", path);
    if !path.exists() {
        Bx::create_path(&path);
    }
}
