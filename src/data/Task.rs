use std::fs;
use std::env;
use std::fmt::format;
use std::io::Write;
use std::path::PathBuf;
use std::process::Child;
use std::thread;
use std::time::Duration;
use std::process::Command;

use reqwest::blocking::Client;
use serde_yaml;
use fs_extra::dir::{copy, CopyOptions};
use fs_extra::dir::DirEntryAttr::Path;
use serde_yaml::Value;

pub struct Task {
    // Task Struktur
    pub name: String,
    pub minservicecount: u32,
    pub maxram: u32,
    pub template: String,
}

pub struct ServerProcess {
    pub name: String,
    pub process: Option<Child>,
}

impl Task {
    // methoden
    pub fn new(name: &str, minservicecount: u32, maxram: u32, template: &str) -> Task {
        let task = Task {
            name: name.to_string(),
            minservicecount,
            maxram,
            template: template.to_string(),
        };
        task
    }

    //get task
    pub fn get_task(name: &str) -> Option<Task> {
        let exe_path = env::current_exe()
            .expect("Ausführungs path konnte nicht gefunden werden");

        let mut task_path = exe_path.clone();
        task_path.pop();
        task_path.push("task");



        // YAML-Dateien lesen
        let yaml_files = fs::read_dir(task_path)
            .expect("Fehler beim Lesen des Task-Ordners")
            .filter_map(|entry| {
                let entry = entry.expect("Fehler beim Lesen des Verzeichniseintrags");
                let file_path = entry.path();
                if file_path.is_file() {
                    Some(file_path)
                } else {
                    None
                }
            });

        // Nach dem Task mit dem angegebenen Namen suchen
        for file_path in yaml_files {

            //read content
            let file_content = fs::read_to_string(&file_path)
                .expect("for schliefe yml task");

            let config: serde_yaml::Value = serde_yaml::from_str(&file_content)
                .expect("Error beim Deserialisieren der config datei");
            //

            let _task_name = config["name"].as_str();

            if let (Some(task_name), Some(minservicecount), Some(maxram), Some(template)) = (
                config["name"].as_str(),
                config["minservicecount"].as_u64(),
                config["maxram"].as_u64(),
                config["template"].as_str(),
            ) {
                if task_name == name {
                    let task = Task::new(task_name, minservicecount as u32, maxram as u32, template);
                    return Some(task);
                }
            }
        }

        None // Wenn kein passender Task gefunden wurde
    }

    //get all task
    pub fn get_task_all() -> Vec<String> {
        let exe_path = env::current_exe()
            .expect("Ausführungs path konnte nicht gefunden werden");

        let mut task_path = exe_path.clone();
        task_path.pop();
        task_path.push("task");

        let mut task_names = Vec::new();

        if task_path.exists() && task_path.is_dir() {
            if let Ok(entries) = fs::read_dir(task_path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Some(file_name) = entry.file_name().to_str() {
                            if let Some(name) = file_name.strip_suffix(".yml") {
                                task_names.push(name.to_string());
                            }
                        }
                    }
                }
            }
        }

        task_names
    }

    //get name
    pub fn get_name(&self) -> &str{
        &self.name
    }

    //get minservicecount
    pub fn get_minservicecount(&self) -> u32{
        self.minservicecount
    }

    //get maxram
    pub fn get_maxram(&self) -> u32{
        self.maxram
    }

    //set ram
    pub fn set_maxram(&self){
        let exe_path = env::current_exe()
            .expect("Ausführungs path konnte nicht gefunden werden");

        let mut task_path = exe_path.clone();
        task_path.pop();
        task_path.push("task");

        // YAML-Dateien lesen
        let yaml_files = fs::read_dir(task_path)
            .expect("Fehler beim Lesen des Task-Ordners")
            .filter_map(|entry| {
                let entry = entry.expect("Fehler beim Lesen des Verzeichniseintrags");
                let file_path = entry.path();
                if file_path.is_file() {
                    Some(file_path)
                } else {
                    None
                }
            });

        // Nach dem Task mit dem angegebenen Namen suchen
        for file_path in yaml_files {

            //read content
            let file_content = fs::read_to_string(&file_path)
                .expect("for schliefe yml task");

            let mut config: serde_yaml::Value = serde_yaml::from_str(&file_content)
                .expect("Error beim Deserialisieren der task datei");

            config["maxram"] = serde_yaml::Value::Number(serde_yaml::Number::from(self.maxram));
        }
    }

    //get template
    pub fn get_template(&self) -> &str{
        &self.template
    }

    //set template
    pub fn set_template(&self){
        let exe_path = env::current_exe()
            .expect("Ausführungs path konnte nicht gefunden werden");

        let mut task_path = exe_path.clone();
        task_path.pop();
        task_path.push("task");

        // YAML-Dateien lesen
        let yaml_files = fs::read_dir(task_path)
            .expect("Fehler beim Lesen des Task-Ordners")
            .filter_map(|entry| {
                let entry = entry.expect("Fehler beim Lesen des Verzeichniseintrags");
                let file_path = entry.path();
                if file_path.is_file() {
                    Some(file_path)
                } else {
                    None
                }
            });

        // Nach dem Task mit dem angegebenen Namen suchen
        for file_path in yaml_files {

            //read content
            let file_content = fs::read_to_string(&file_path)
                .expect("for schliefe yml task");

            let mut config: serde_yaml::Value = serde_yaml::from_str(&file_content)
                .expect("Error beim Deserialisieren der task datei");

            config["template"] = serde_yaml::Value::String(self.template.clone());
        }
    }

    pub fn create(&self) -> bool{
        let mut exe_path:PathBuf = env::current_exe().expect("Fehler beim Abrufen des Ausführungspfads");
        exe_path.pop();
        let mut task_path = exe_path.clone();
        task_path.push("task");
        task_path.push(format!("{}.yml", &self.name));
        task_path.to_str().expect("Error beim confertiren des task_path in einen String").to_string();

        let mut config_task_path = exe_path.clone();
        config_task_path.push("config");
        config_task_path.push("task.yml");
        config_task_path.to_str().expect("Error beim confertiren des config_task_path in einen String").to_string();

        //lesen des inhalts der deault task datei
        let default_file_content = fs::read_to_string(config_task_path);

        // YAML-Wert aus dem Inhalt erstellen
        let binding = default_file_content.expect("Error beim default_file_content_str");
        let default_file_content_str: &str = &binding.as_str();

        let mut new_content: Value = match serde_yaml::from_str(default_file_content_str) {
            Ok(value) => value,
            Err(error) => {
                println!("Fehler beim Konvertieren des Inhalts in einen YAML-Wert: {}", error);
                return false;
            }
        };

        // Werte ersetzen
        new_content["name"] = serde_yaml::Value::String(self.name.clone());
        //new_content["minservicecount"] = serde_yaml::Value::Number(serde_yaml::Number::from(self.minservicecount));
        //new_content["maxram"] = serde_yaml::Value::Number(serde_yaml::Number::from(self.maxram));
        new_content["template"] = serde_yaml::Value::String(self.template.clone());

        let mut file = match fs::File::create(task_path) {
            Ok(file) => file,
            Err(error) => {
                println!("Fehler beim Erstellen der neuen Task-Datei: {}", error);
                return false;
            }
        };

        // Neue YAML-Datei erstellen
        let yaml_string = match serde_yaml::to_string(&new_content) {
            Ok(string) => string,
            Err(error) => {
                println!("Fehler beim Konvertieren des YAML-Werts in einen String: {}", error);
                return false;
            }
        };

        if let Err(error) = file.write_all(yaml_string.as_bytes()) {
            println!("Fehler beim Schreiben in die Task-Datei: {}", error);
            return false;
        }
        return true;
    }
    pub fn start_as_service(self) {
        println!("start_as_serice");

        let exe_path = env::current_exe()
            .expect("Error beim lesen des exe path");

        let mut template_path = exe_path.clone();
        template_path.pop();
        template_path.push("template");
        template_path.push(&self.template);

        println!("{:?}", template_path);

        let mut service_path = exe_path.clone();
        service_path.pop();
        service_path.push("service");
        service_path.push("temp");

        println!("{:?}", service_path);

        //-----------------------

        //gloabel hier??

        //-----------------------


        if service_path.exists() && service_path.is_dir(){
            fs::remove_dir_all(&service_path).expect("Error beim löschen des temp ordners");
            fs::create_dir_all(&service_path).expect("Error beim erstellen des temp ordners");
        }

        let mut options = CopyOptions::new();
        options.overwrite = true;

        match copy(&template_path, &service_path, &options) {
            Ok(_) => println!("Ordner erfolgreich kopiert"),
            Err(e) => println!("Fehler beim Kopieren des Ordners: {}", e),
        };

        let mut server_dir_name = format!("{}-1", &self.name);
        let mut i = 1;

        while Path::new(&server_dir_name).exists() {
            i += 1;
            server_dir_name = format!("{}-{}", &self.name, i);
        }

        //rename the folder
        match fs::rename(&template_path, new_folder_name) {
            Ok(_) => println!("Ordner erfolgreich umbenannt."),
            Err(e) => println!("Fehler beim Umbenennen des Ordners: {}", e),
        }

        thread::sleep(Duration::from_secs(5));

        //--------------------------------------

        //umbennen des server ordners
        // bearbeitung der configs
        //vllt noch hier den gloabl importieren

        //---------------------------------------

        service_path.push("server");

        let mut service_path_jar = service_path.clone();
        service_path_jar.push("paper.jar");

        let service_path_string_jar = service_path_jar.to_str().expect("Fehler beim Konvertieren des Jar-Pfads").to_string();
        let service_path_string = service_path.to_str().expect("Fehler beim Konvertieren des Pfads").to_string();

        //start the server
        let server = Command::new("java")
            .args(&[format!("-Xmx{}G", &self.maxram), "-jar", &service_path_string_jar])
            .current_dir(&service_path_string)
            .spawn()
            .expect("Fehler beim Starten des Servers");


        //webhook anfrage an den proxy
        let webhook_url = "http://localhost:8000/webhook"; // Passe die URL entsprechend an

        // Erstelle eine reqwest-Clientinstanz
        let client = Client::new();

        // Definiere die Daten, die du an das Java-Plugin senden möchtest
        let data = [
            ("name", &self.name),
            ("host", &"127.0.0.1".to_string()),
            ("port", &"25555".to_string()),
        ];

        // Führe die HTTP-POST-Anfrage durch
        match client.post(webhook_url).form(&data).send() {
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
    }


}

