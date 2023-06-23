use std::fs;
use std::env;
use std::process::Command;
use std::io;
use std::fs::File;

use std::io::Write;

use std::path::Path;
use std::path::PathBuf;
use std::task;

use reqwest;
use reqwest::blocking::get;

use serde_yaml;
use serde::Deserialize;
use fs_extra::dir::{copy, CopyOptions};
pub struct Task {
    // Task Struktur
    pub name: String,
    pub minservicecount: u32,
    pub maxram: u32,
    pub template: String
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
            
            let task_name = config["name"].as_str();

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

    //get template
    pub fn get_template(&self) -> &str{
        &self.template
    }


    pub fn start_as_service(&self) {
        println!("start_as_serice");
        println!();
        // Implementiere hier deine Logik zum Starten des Servers als Service
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


        if service_path.exists() && service_path.is_dir(){
            fs::remove_dir_all(&service_path).expect("Error beim löschen des temp ordners");
        }


        let mut options = CopyOptions::new();
        options.overwrite = true;
    
        match copy(template_path, service_path, &options) {
            Ok(_) => println!("Ordner erfolgreich kopiert"),
            Err(e) => println!("Fehler beim Kopieren des Ordners: {}", e),
        };

        //start mc server

        let output = Command::new("java")
            .arg("-Xmx1G")
            .arg("velo.jar")
            .output()
            .expect("Error beim starten des servers");
        
        if output.status.success(){
            println!("Server gestartet");
        }else {
            println!("Eooro beim starten des servers if");
        }





        

    }
    
}
