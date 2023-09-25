use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use colored::*;
use reqwest::blocking::get;
use serde_json::Value;
use crate::config::Config;
use crate::data::task::Task;
use crate::lib::bx::Bx;

pub struct Starting;

impl Starting {
    pub fn start(exe_path: PathBuf) -> bool{
        Starting::print_icon();

        if let Some(config) = Starting::check_config(&exe_path){

            let cmd_prefix = Config::get_prefix();
            Starting::check_folder(&exe_path, &config, &cmd_prefix);

            if Starting::check_link(&exe_path, &config, &cmd_prefix){
                Starting::check_task();

            } else {
                return false;
            }

        } else {
            return false;
        }
        return true;
    }

    fn check_task(){
        let task_all = Task::get_task_all();

        println!("{:?}", task_all);

        for task_name in task_all {
            if let Some(task) = Task::get_task(task_name) {
                println!("{}", &task.get_name());
                if task.get_min_service_count() > 0 {
                    for _ in 0..task.get_min_service_count() {
                        println!("Dienst starten {}", &task.get_name());
                        //task.prepared_to_services();
                    }
                }
            } else {
                println!("{} task error", Config::get_prefix());
            }
        }

    }

    fn print_icon() {
        println!("");
        println!("_____{}__________________________________________________________{}__{}________________________________________{}__", r"/\\\\\\\\\\\\".red(), r"/\\\\\\\\\".cyan(), r"/\\\\\\".cyan(), r"/\\\".cyan() );
        println!("___{}________________________________________________________{}__{}_______________________________________{}__", r"/\\\//////////".red(), r"/\\\////////".cyan(), r"\////\\\".cyan(), r"\/\\\".cyan() );
        println!("__{}_________________________________________________________________{}______________{}_______________________________________{}__", r"/\\\".red(), r"/\\\/".cyan(), r"\/\\\".cyan(), r"\/\\\".cyan() );
        println!("_{}____{}__{}_______{}__{}_______{}___{}________________{}________{}_____{}____{}________{}__", r"\/\\\".red(), r"/\\\\\\\".red(), r"/\\\\\\\\\".red(), r"/\\\\\".red(), r"/\\\\\".red(), r"/\\\\\\\\".red(), r"/\\\".cyan(), r"\/\\\".cyan(), r"/\\\\\".cyan(), r"/\\\".cyan(), r"/\\\".cyan(), r"\/\\\".cyan());
        println!("_{}___{}_{}____{}___{}_{}________________{}______{}__{}___{}___{}__", r"\/\\\".red(), r"\/////\\\".red(), r"\////////\\\".red(), r"/\\\///\\\\\///\\\".red(), r"/\\\/////\\\".red(), r"\/\\\".cyan(), r"\/\\\".cyan(), r"/\\\///\\\".cyan(), r"\/\\\".cyan(), r"\/\\\".cyan(), r"/\\\\\\\\\".cyan());
        println!("__{}_______{}___{}__{}_{}__{}__{}__{}_______________{}_____{}__{}_{}___{}__{}__", r"\/\\\".red(), r"\/\\\".red(), r"/\\\\\\\\\\".red(), r"\/\\\".red(), r"\//\\\".red(), r"\/\\\".red(), r"/\\\\\\\\\\\".red(), r"\//\\\".cyan(), r"\/\\\".cyan(), r"/\\\".cyan(), r"\//\\\".cyan(), r"\/\\\".cyan(), r"\/\\\".cyan(), r"/\\\////\\\".cyan());
        println!("___{}_______{}__{}__{}__{}__{}_{}____{}_____________{}____{}__{}__{}___{}_{}__{}__", r"\/\\\".red(), r"\/\\\".red(), r"/\\\/////\\\".red(), r"\/\\\".red(), r"\/\\\".red(), r"\/\\\".red(), r"\//\\///////".red(), r"\///\\\".cyan(), r"\/\\\".cyan(), r"\//\\\".cyan(), r"/\\\".cyan(), r"\/\\\".cyan(), r"\/\\\".cyan(), r"\/\\\".cyan(), r"\/\\\".cyan());
        println!("____{}__{}_{}__{}__{}__{}____{}__{}__{}___{}__{}_", r"\//\\\\\\\\\\\\/".red(), r"\//\\\\\\\\/\\".red(), r"\/\\\".red(), r"\/\\\".red(), r"\/\\\".red(), r"\//\\\\\\\\\\".red(), r"\////\\\\\\\\\".cyan(), r"/\\\\\\\\\".cyan(), r"\///\\\\\/".cyan(), r"\//\\\\\\\\\".cyan(), r"\//\\\\\\\/\\".cyan());
        println!("_____{}_____{}__{}___{}___{}____{}________{}__{}_____{}______{}____{}__", r"\////////////".red(), r"\////////\//".red(), r"\///".red(), r"\///".red(), r"\///".red(), r"\//////////".red(), r"\/////////".cyan(), r"\/////////".cyan(), r"\/////".cyan(), r"\/////////".cyan(), r"\///////\//".cyan());
        println!("");
    }

    fn check_config(exe_path: &PathBuf) -> Option<Value>{

        //config.yml
        let mut config_file_path = exe_path.clone();
        config_file_path.push("config.json");

        if !config_file_path.exists() {
            let url = "http://thephoenixpixel.de/cloud/game_cloud/config.json";
            if let Some(response) = get(url).ok() {
                let mut file = File::create(&config_file_path);
                file.expect("Error beim write all config.json")
                    .write_all(&*response.bytes().expect("dwdw"))
                    .expect("Error beim schreiben der datei");

            } else {
                eprintln!("Cloud kann nicht starten");
                eprintln!("Das System kann die url {} nicht abrufen", url);
                return None;
            }

             println!("Datei erstellt von {}", url);
        }

        //config.json dereralisseiren
        let config_content = fs::read_to_string(&config_file_path).expect("Error beim lesen des config content");

        Some(serde_json::from_str(&config_content).expect("Error beim dereraliesiren des config inhalts"))

    }

    fn check_folder(exe_path: &PathBuf, config: &Value, cmd_prefix: &ColoredString){

        //task folder
        {
            let mut task_path = exe_path.clone();
            task_path.push(config["path"]["task"].as_str().expect("Error beim Lesen des path der config datei"));
            if !task_path.exists() {
                Bx::create_path(&task_path);
                println!("{} Task ordner erfolgreich erstellt {:?}", cmd_prefix, task_path);
            }
        }

        //template folder
        {
            let mut template_path = exe_path.clone();
            template_path.push(config["path"]["template"].as_str().expect("Error beim Lesen des path der config datei"));
            if !template_path.exists() {
                Bx::create_path(&template_path);
                println!("{} {:?} erfolgreich erstellt",cmd_prefix ,template_path);
            }
        }

        //service temp folder
        {
            let mut service_temp_path = exe_path.clone();
            service_temp_path.push(config["path"]["service"]["temp"].as_str().expect("Error beim Lesen des path der config datei"));
            if !service_temp_path.exists() {
                Bx::create_path(&service_temp_path);
                println!("{} {:?} erfolgreich erstellt",cmd_prefix ,service_temp_path);
            }
        }

        //service static folder
        {
            let mut service_static_path = exe_path.clone();
            service_static_path.push(config["path"]["service"]["static"].as_str().expect("Error beim Lesen des path der config datei"));
            if !service_static_path.exists() {
                Bx::create_path(&service_static_path);
                println!("{} {:?} erfolgreich erstellt",cmd_prefix ,service_static_path);
            }
        }

    }

    fn check_link(exe_path: &PathBuf, config: &Value, cmd_prefix: &ColoredString) -> bool {
        //software.json link
        {
            let mut config_software_path = exe_path.clone();
            config_software_path.push(config["path"]["config"]["software"].as_str().expect("Error beim Lesen des path der config datei"));
            Bx::create_path(&config_software_path);
            println!("{} {:?} erfolgreich erstellt",cmd_prefix ,config_software_path);
            config_software_path.push("links.json");
            if !config_software_path.exists() {
                let url = "http://thephoenixpixel.de/cloud/game_cloud/config/software.json";
                if let Some(response) = get(url).ok() {
                    let mut file = File::create(&config_software_path);
                    file.expect("Erro beim ersetllend er File").write_all(&*response.bytes().expect("Error beim schreiben der File"));
                    println!("{} Datei erstellt von {}",cmd_prefix ,url);

                } else {
                    eprintln!("Software file kann nicht heruntergeladen werden");
                    eprintln!("Bitte stellen sie sicher das sie zugriff auf {} haben", url);
                    return false;
                }
            }
        }
        //task.json link
        {
            let mut config_task_path = exe_path.clone();
            config_task_path.push(config["path"]["config"]["default_task"].as_str().expect("Error beim Lesen des path der config datei"));
            Bx::create_path(&config_task_path);
            println!("{} {:?} erfolgreich erstellt",cmd_prefix ,config_task_path);
            config_task_path.push("task.json");
            if !config_task_path.exists() {
                let url = "http://thephoenixpixel.de/cloud/game_cloud/config/task.json";
                if let Some(response) = get(url).ok() {
                    let mut file = File::create(&config_task_path);
                    file.expect("Erro beim ersetllend er File").write_all(&*response.bytes().expect("Error beim schreiben der File"));
                    println!("{} Datei erstellt von {}",cmd_prefix ,url);

                } else {
                    eprintln!("task default file kann nicht heruntergeladen werden");
                    eprintln!("Bitte stellen sie sicher das sie zugriff auf {} haben", url);
                    return false;
                }
            }
        }
        return true;
    }
}
