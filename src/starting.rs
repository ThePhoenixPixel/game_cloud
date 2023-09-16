use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use reqwest::blocking::get;
use serde_json::Value;
use crate::config::Config;
use crate::lib::bx::Bx;

pub struct Starting;

impl Starting {
    pub fn check_path(exe_path: PathBuf) {
        //config.yml
        let mut config_file_path = exe_path.clone();
        config_file_path.push("config.json");
        {
            if !config_file_path.exists() {
                let url = "http://thephoenixpixel.de/cloud/game_cloud/config.json";
                let response = get(url).expect("Error can't get url").text();
                let mut file = File::create(&config_file_path);
                file.expect("Error beim write all config.json").write_all(response.expect("Error eim response ").as_bytes()).expect("Error beim schreiben der datei");
                println!("Datei erstellt von {}", url);
            }
        }
        //config.json dereralisseiren
        let config_content = fs::read_to_string(&config_file_path).expect("Error beim lesen des config content");
        let mut config: Value = serde_json::from_str(&config_content).expect("Error beim dereraliesiren des config inhalts");
        let cmd_präfix = Config::get_prefix();

        //task dir
        {
            let mut task_path = exe_path.clone();
            task_path.push(config["path"]["task"].as_str().expect("Error beim Lesen des path der config datei"));
            Bx::create_path(&task_path);
            println!("{} Task ordner erfolgreich erstellt {:?}", cmd_präfix, task_path);
        }
        //template dir
        {
            let mut template_path = exe_path.clone();
            template_path.push(config["path"]["template"].as_str().expect("Error beim Lesen des path der config datei"));
            Bx::create_path(&template_path);
            println!("{} {:?} erfolgreich erstellt",cmd_präfix ,template_path);
        }
        //service temp dir
        {
            let mut service_temp_path = exe_path.clone();
            service_temp_path.push(config["path"]["service"]["temp"].as_str().expect("Error beim Lesen des path der config datei"));
            Bx::create_path(&service_temp_path);
            println!("{} {:?} erfolgreich erstellt",cmd_präfix ,service_temp_path);
        }
        //service static dir
        {
            let mut service_static_path = exe_path.clone();
            service_static_path.push(config["path"]["service"]["static"].as_str().expect("Error beim Lesen des path der config datei"));
            Bx::create_path(&service_static_path);
            println!("{} {:?} erfolgreich erstellt",cmd_präfix ,service_static_path);
        }
        //config links
        {
            let mut config_links_path = exe_path.clone();
            config_links_path.push(config["path"]["config"]["links"].as_str().expect("Error beim Lesen des path der config datei"));
            Bx::create_path(&config_links_path);
            println!("{} {:?} erfolgreich erstellt",cmd_präfix ,config_links_path);
            config_links_path.push("links.json");
            if !config_links_path.exists() {
                let url = "http://thephoenixpixel.de/cloud/game_cloud/config/links.json";
                let response = get(url).expect("Error can't get url").text();

                let mut file = File::create(&config_links_path);
                file.expect("Erro beim ersetllend er File").write_all(response.expect("Error beim lesen des response").as_bytes()).expect("Error beim schreiben der File");

                println!("{} Datei erstellt von {}",cmd_präfix ,url);
            }

        }
        //config default_task
        {
            let mut config_task_path = exe_path.clone();
            config_task_path.push(config["path"]["config"]["links"].as_str().expect("Error beim Lesen des path der config datei"));
            Bx::create_path(&config_task_path);
            println!("{} {:?} erfolgreich erstellt",cmd_präfix ,config_task_path);
            config_task_path.push("task.json");
            if !config_task_path.exists() {
                let url = "http://thephoenixpixel.de/cloud/game_cloud/config/task.json";
                let response = get(url).expect("Error can't get url").text();

                let mut file = File::create(&config_task_path);
                file.expect("Erro beim ersetllend er File").write_all(response.expect("Error beim lesen des response").as_bytes()).expect("Error beim schreiben der File");

                println!("{} Datei erstellt von {}",cmd_präfix ,url);
            }

        }
    }
}
