use std::fs;
use std::fs::File;
use std::io::Write;
use std::iter::Cycle;
use std::mem::replace;
use std::path::PathBuf;
use colored::Colorize;
use colored::*;
use reqwest::blocking::get;
use serde_json::Value;
use crate::config::Config;
use crate::lib::bx::Bx;

pub struct Starting;

impl Starting {
    pub fn print_icon() {
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

    /*println!(r"#_____/\\\\\\\\\\\\__________________________________________________________/\\\\\\\\\__/\\\\\\________________________________________/\\\__
                 #___/\\\//////////________________________________________________________/\\\////////__\////\\\_______________________________________\/\\\__
                 #__/\\\_________________________________________________________________/\\\/______________\/\\\_______________________________________\/\\\__
                 #_\/\\\____/\\\\\\\__/\\\\\\\\\_______/\\\\\__/\\\\\_______/\\\\\\\\___/\\\________________\/\\\________/\\\\\_____/\\\____/\\\________\/\\\__
                 #_\/\\\___\/////\\\_\////////\\\____/\\\///\\\\\///\\\___/\\\/////\\\_\/\\\________________\/\\\______/\\\///\\\__\/\\\___\/\\\___/\\\\\\\\\__
                 #__\/\\\_______\/\\\___/\\\\\\\\\\__\/\\\_\//\\\__\/\\\__/\\\\\\\\\\\__\//\\\_______________\/\\\_____/\\\__\//\\\_\/\\\___\/\\\__/\\\////\\\__
                 #___\/\\\_______\/\\\__/\\\/////\\\__\/\\\__\/\\\__\/\\\_\//\\///////____\///\\\_____________\/\\\____\//\\\__/\\\__\/\\\___\/\\\_\/\\\__\/\\\__
                 #____\//\\\\\\\\\\\\/__\//\\\\\\\\/\\_\/\\\__\/\\\__\/\\\__\//\\\\\\\\\\____\////\\\\\\\\\__/\\\\\\\\\__\///\\\\\/___\//\\\\\\\\\__\//\\\\\\\/\\_
                 #_____\////////////_____\////////\//__\///___\///___\///____\//////////________\/////////__\/////////_____\/////______\/////////____\///////\//__");*/




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
            let mut config_software_path = exe_path.clone();
            config_software_path.push(config["path"]["config"]["software"].as_str().expect("Error beim Lesen des path der config datei"));
            Bx::create_path(&config_software_path);
            println!("{} {:?} erfolgreich erstellt",cmd_präfix ,config_software_path);
            config_software_path.push("links.json");
            if !config_software_path.exists() {
                let url = "http://thephoenixpixel.de/cloud/game_cloud/config/software.json";
                let response = get(url).expect("Error can't get url").text();

                let mut file = File::create(&config_software_path);
                file.expect("Erro beim ersetllend er File").write_all(response.expect("Error beim lesen des response").as_bytes()).expect("Error beim schreiben der File");

                println!("{} Datei erstellt von {}",cmd_präfix ,url);
            }

        }
        //config default_task
        {
            let mut config_task_path = exe_path.clone();
            config_task_path.push(config["path"]["config"]["default_task"].as_str().expect("Error beim Lesen des path der config datei"));
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
