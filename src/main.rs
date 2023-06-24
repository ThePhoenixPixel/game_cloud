use std::fs;
use std::env;
use crate::data::task::Task;

/*
use std::f32::consts::E;
use reqwest;
use reqwest::blocking::get;
use bib::dir::Dir;
use data::group;
use data::service;
mod bib{
    pub mod dir;
}
*/
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

    let task = Task::get_task("Proxy").expect("jou error bei get task");
    
    println!("Name: {}", task.get_name());
    println!("Min Service Count: {}", task.get_minservicecount());
    println!("Max Ram: {}", task.get_maxram());
    println!("Template: {}", task.get_template());


    task.start_as_service();


    //end
    println!("BB");
    Ok(())
}



/*if !cloud_file_config_path.exists() {
        let url = "http://dev.phoenixcraft.eu/cloud/config.yml";
        let response = get(url)?.text()?;
        
        let mut file = File::create(&cloud_file_config_path)?;
        file.write_all(response.as_bytes())?;
        
        println!("Datei erstellt von {}", url);
    }*/