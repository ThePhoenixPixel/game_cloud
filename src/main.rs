use std::fs;
use std::env;
use std::io;
use std::fs::File;

use std::io::Write;

use std::path::Path;

use reqwest;
use reqwest::blocking::get;

use serde_yaml;
use serde::Deserialize;

/*use std::f32::consts::E;

use bib::dir::Dir;
use data::group;

use data::service;

*/

use data::task;

use crate::data::task::Task;

mod bib{
    pub mod dir;
}

mod data{
    pub mod task;
    pub mod service;
    pub mod group;
}


fn main() -> Result<(), Box<dyn std::error::Error>>{

    println!("Start Game Cloud");

    //read file and dirs
    let exe_path = env::current_exe().expect("Fehler beim Abrufen des Ausführungspfads");
    let mut task_path = exe_path.clone();
    task_path.pop();
    task_path.push("task");

    if !task_path.exists() {
        // Ordner erstellen, falls nicht vorhanden
        fs::create_dir(&task_path).expect("Fehler beim Erstellen des Task-Ordners");
    }

    let task = Task::get_task("Lobby").expect("jou error bei get task");
    
    println!("Name: {}", task.get_name());
    println!("Min Service Count: {}", task.get_minservicecount());
    println!("Max Ram: {}", task.get_maxram());
    println!("Template: {}", task.get_template());


    task.start_as_service();


    //end
    let mut test = String::new();
    io::stdin().read_line(&mut test);
    println!("BB");
    Ok(())

}




fn find_yaml_files(folder_path: &str) -> Vec<String> {
    let mut yaml_files = Vec::new();

    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if file_path.is_file() && file_path.extension().and_then(|ext| ext.to_str()) == Some("yml") {
                    if let Some(file_name) = file_path.file_name().and_then(|name| name.to_str()) {
                        yaml_files.push(file_name.to_owned());
                    }
                }
            }
        }
    }

    return yaml_files;
}

/*if !cloud_file_config_path.exists() {
        let url = "http://dev.phoenixcraft.eu/cloud/config.yml";
        let response = get(url)?.text()?;
        
        let mut file = File::create(&cloud_file_config_path)?;
        file.write_all(response.as_bytes())?;
        
        println!("Datei erstellt von {}", url);
    }*/