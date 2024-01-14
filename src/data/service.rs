use std::{fs, thread};
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;
use chrono::{DateTime, Local};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::data::task::Task;
use crate::lib::address::Address;
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
    plugin_listener: Address,
    cloud_listener: Address,
    task: String,
}

impl Service {

    pub fn new(service_name: &String, task: &Task) -> Service {
        let temp_port:Option<u32> = Config::get_node_port().try_into().ok();
        let mut port:u32 = 0;
        match temp_port {
            Some(result) => {
                port = result;
            }
            None => {
                println!("umwandlung nicht möglich");
            }
        }

        let plugin_listener = Address::new(&"127.0.0.1".to_string(), &Address::find_next_port(&"127.0.0.1".to_string(), task.get_start_port()));
        let cloud_listener = Address::new(&Config::get_node_host(), &port);

        Service {
            name: service_name.clone(),
            status: "Stop".to_string(),
            time: Local::now(),
            plugin_listener,
            cloud_listener,
            task: task.get_name(),
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
        #[warn(unused_assignments)]
        let mut path = PathBuf::new();
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
        println!("{} Start {} Service", Config::get_prefix(), service_name);

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
            println!("Error beim getten des service")
        }
    }
}

fn start(task: &Task, service: &Service, path: &PathBuf){

    let port = Address::find_next_port(&Config::get_server_host(),task.get_start_port());
    let task_clone = task.clone();
    let path_clone = path.clone();

    thread::spawn(move || {
        // Öffne Dateien für die Ausgabeumleitung
        let stdout_file = File::create("server_stdout.log").expect("Fehler beim Erstellen der Ausgabedatei");
        let stderr_file = File::create("server_stderr.log").expect("Fehler beim Erstellen der Fehlerausgabedatei");

        // Öffne die Datei für die Standardeingabe (stdin) und lese nichts daraus
        let stdin_file = File::open("/dev/null").expect("Fehler beim Öffnen der Standardeingabe");
        println!("{}", path_clone.join(task_clone.get_software().get_name_with_ext()).to_str().unwrap().to_string());
        // Code zum Server starten
        let _server = Command::new("java")
            .args(&[
                format!("-Xmx{}M", task_clone.get_max_ram().to_string()),
                "-jar".to_string(),
                path_clone.join(task_clone.get_software().get_name_with_ext()).to_str().unwrap().to_string(),
            ])

            .current_dir(path_clone)
            .stdout(Stdio::from(stdout_file))
            .stderr(Stdio::from(stderr_file))
            .stdin(Stdio::from(stdin_file))  // Hier wird die Standardeingabe (stdin) umgeleitet
            .spawn()
            .expect("Fehler beim Starten des Servers");

        // Warte auf das Beenden des Servers, wenn notwendig
        // server.wait().expect("Fehler beim Warten auf den Server");
    });


    //connect to preoxy
    if task.get_software().get_name().clone() == "paper".to_string() {
        if let Some(_) = send_webhook_request(&service.get_name(), &Config::get_server_host(), &port).ok() {
            println!("erfolgreich gesendet");
        } else {
            println!("error bei der anfrage an den proxy");
        }
    }

}

fn send_webhook_request(server_name: &String, ip: &String, port: &u32) -> Result<(), reqwest::Error> {
    let duration = Duration::from_secs(3);
    thread::sleep(duration);
    // Ersetze diese URL durch die tatsächliche URL deines Webhook-Servers
    let url = format!("http://{}:{}/service/{}", ip, port, "Proxy-1");
    println!("url ist: {}", url);


    let client = Client::new();
    let mut try_to_connect = String::new();
    if server_name.starts_with("Lobby") {
        try_to_connect = String::from("true");
    } else {
        try_to_connect = String::from("false");
    }

    // Erstelle die Daten, die du im Webhook-Request senden möchtest
    let data = [("name", server_name), ("try_to_connect", &try_to_connect), ("host", ip), ("port", &port.to_string())];

    match client.post(url).form(&data).send() {
        Ok(response) => {
            if response.status().is_success() {
                println!("Webhook-Anfrage erfolgreich gesendet.");
                // Hier kannst du die Antwort des Java-Plugins verarbeiten, falls gewünscht.
            } else {
                println!("Fehler beim Senden der Webhook-Anfrage: {:?}", response.status());
            }
        }
        Err(e) => {
            println!("Fehler beim Senden der Webhook-Anfrage: {:?}", e);
        }
    }

    Ok(())
}


fn prepare_to_start(service_path: &mut PathBuf, task: &Task) {
    if !is_service_start(service_path) {
        if !service_path.exists() {
            task.prepared_to_services()
        }
        service_path.pop();
        service_path.pop();
        check_folder(service_path, task);
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

fn check_folder(path: &mut PathBuf, task: &Task) {
    //println!("in check folder {:?}", path);
    path.push(".game_cloud");
    create_service_folder(path);
    create_service_file(path, task);
}

fn create_service_file(path: &mut PathBuf, task: &Task) {
    path.push("service_config.json");
    if !path.exists() { // Wenn die Datei nicht existiert, erstelle sie
        let mut service_path = path.clone();
        service_path.pop();
        service_path.pop();

        let service = Service::new(&Bx::get_last_folder_name(&service_path), task);
        let default_config_str = serde_json::to_string_pretty(&service).expect("Fehler beim Serialisieren der Standardkonfiguration");
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


/*

fn extract_and_install_links(software_type: &str, software_links: &Map<String, Value>){
    //let install_dir = Config::get_software_files_path();
    let cmd_prefix = Config::get_prefix();

    // Iteriere durch die Kategorien (self, server, proxy)
    // Iteriere durch die Software-Links in diesem Software-Typ (Kategorie)
    for (software_name, software_link_value) in software_links.iter() {
        if let Some(software_link) = software_link_value.as_str() {
            //let software_dir = install_dir.join(software_type);
            install_software(software_link, software_name, software_type, &cmd_prefix);

        } else {
            println!(
                "{} Ungültiger Link für Software: {}",
                cmd_prefix, software_name
            );
        }
    }
*/