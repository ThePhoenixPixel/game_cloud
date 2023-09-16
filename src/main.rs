use std::env;
use std::io::{Read, Write};
use std::path::PathBuf;
use colored::Colorize;
use serde_json;
use crate::cmd::cmd::Cmd;
use crate::config::Config;
use crate::data::task::Task;
use crate::starting::Starting;

pub mod language;
pub mod starting;
pub mod lib{
    pub mod bx;
}

pub mod cmd{
    pub mod cmd;
    pub mod command{
        //pub mod cmd_group;
        //pub mod cmd_node;
        //pub mod cmd_software;
        pub mod cmd_stop;
        pub mod cmd_task;
        //pub mod cmd_template;
    }
}
pub mod data{
    pub mod task;
    pub mod template;
    //pub mod group;
    //pub mod node;
    pub mod software;
    pub mod service;
}

mod config;

fn main(){
    println!("Start Game Cloud");
    let dev_mode = false;
    //read file and dirs
    let mut exe_path:PathBuf = env::current_exe().expect("Fehler beim Abrufen des Ausführungspfads");
    exe_path.pop();

    Starting::check_path(exe_path);

    //task abfrage und initaliesirung der
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

    // cmd
    let mut cmd = Cmd::new().start();

    //end
    println!("{} BB", Config::get_prefix());
}






fn user_input(args: &[&str]) -> bool{
    if let Some(command) = args.get(0) {
        match command {
            &"stop" => {
                println!("stop");



                return false;
            }
            &"task"=> {
                println!("task");
                if let Some(sub0) = args.get(1) {
                    match sub0 {
                        &"create" => {
                            println!("task create");

                            if let Some(sub1) = args.get(2) {


                                println!("task create {}", sub1);

                                if let Some(sub2) = args.get(3) {

                                    match sub2 {

                                        &"Proxy" => {
                                            println!("task create {} proxy", sub1);
                                            println!("task erfolgreich erstellt (proxy)");

                                            let mut task = Task::new();
                                            task.set_name(sub2.to_string());
                                            task.save_to_file();

                                        }
                                        &"Server" => {
                                            println!("task create {} server", sub1);
                                            println!("task erfolgreich erstellt (Servertask )");

                                            let mut task = Task::new();
                                            task.set_name(sub1.to_string());
                                            task.save_to_file();

                                        }
                                        _ => {
                                            println!("dies ist kein gültiger typ")
                                        }
                                    }
                                } else {
                                    println!("Bitte gebe ein typ ein <server/proxy>");
                                }
                            } else {
                                println!("Bitte gebe einen namen für die Task ein");
                                println!("task create <name> <server/Proxy>");
                            }
                        }
                        &"setup" => {
                            println!("{} ", Config::get_prefix())
                        }

                        &"delete" => {
                            println!("task delete");
                            if let Some(sub1) = args.get(2) {

                            } else{
                                println!("Bitte gebe einen namen der Task ein den sie deleten wollen");
                                println!("task delete <name>");
                            }
                        }
                        &"setup" => {
                            println!("task setup");
                            if let Some(sub1) = args.get(2) {
                                match sub1 {
                                    _ => {
                                        println!("flascher ")
                                    }
                                }
                            }else {
                                println!("Bitte gebe einen namen der Task ein den sie deleten wollen");
                                println!("task delete <name>");
                            }
                        }
                        _ => {
                            println!("diesen sub command gibt es nicht");
                            println!("Use << help >> um dir die Commands anzeigen zu lassen");
                        }
                    }
                } else {
                    println!("task create <name> <Server/Proxy>");
                    println!("task delete <name>");
                }
            }

            &"help" => {
                println!("stop -> stoppen der cloud und damit allen services die gestartet sind");
                println!("help -> anzeigen von den commands");
                println!("task -> bearbeiten der tasks");
                println!("service -> interagiren mit den services");
            }

            &"service" => {
                println!("service");
                if let Some(sub0) = args.get(1){
                    match sub0 {
                        &"start" => {
                            if let Some(sub1) = args.get(2){
                                //let service = Service::new(sub1);
                                //service.start();
                            }else {
                                println!("bitte gebe ein task name ein");
                                println!("damit dieser als service starten kann");
                            }

                        }
                        _ => {
                            println!("diesen sub command gibt es nicht");
                            println!("Use << help >> um dir die Commands anzeigen zu lassen");
                        }
                    }
                }else {
                    println!("service start <task_name>");
                }
            }
            _ => {
                println!("Ich wusste garnicht das es diesen Command gibt");
                println!("Benutze doch einfach einen von denen ->");
                println!("Use << help >> um sie dir anzeigen zu lassen");
            }
        }
    }else {
        print!("kein Command");
    }
    return true;
}




















/*

    //config dir
    {

        let mut config_path = Path::new(config["path"]["config"]["links"].as_str().expect("Error beim "));
        config_path.push("config");
        if !config_path.exists() {
            // Ordner erstellen, falls nicht vorhanden
            fs::create_dir(&config_path).expect("Fehler beim Erstellen des config-Ordners");
        }

        //task.yml
        {
            let mut task_default_path = config_path.clone();
            task_default_path.push("task.yml");
    
            if !task_default_path.exists() {
                let url = "http://thephoenixpixel.de/cloud/game_cloud/config/task.yml";
                let response = get(url).expect("Error can't get url").text();
    
                let mut file = File::create(&task_default_path);
                file.expect("Erro beim ersetllend er File").write_all(response.expect("Error beim lesen des response").as_bytes()).expect("Error beim schreiben der File");
    
                println!("Datei erstellt von {}", url);
            }
        }
        //links.yml
        {
            let mut links_default_path = config_path.clone();
            links_default_path.push("links.yml");

            if !links_default_path.exists() {
                let url = "http://thephoenixpixel.de/cloud/game_cloud/config/links.yml";
                let response = get(url).expect("Error can't get url").text();

                let mut file = File::create(&links_default_path);
                file.expect("Erro beim ersetllend er File").write_all(response.expect("Error beim lesen des response").as_bytes()).expect("Error beim schreiben der File");

                println!("Datei erstellt von {}", url);
            }
        }
    }

    //task
    {
        let mut task_path = exe_path.clone();
        task_path.push("task");
        if !task_path.exists() {
            // Ordner erstellen, falls nicht vorhanden
            fs::create_dir(&task_path).expect("Fehler beim Erstellen des Task-Ordners");
        }
    }

    //template
    {
        let mut template_path = exe_path.clone();
        template_path.push("template");
        if !template_path.exists(){
            fs::create_dir(&template_path)
                .expect("Error beim erstellen des Template Ordners");
        }

    }

    //service
    {
        let mut service_path = exe_path.clone();
        service_path.push("service");
        if !service_path.exists() {
            fs::create_dir(&service_path).expect("Error beim erstellen des service folders");
        }
        //service/temp
        {
            let service_temp_path_str = config["path"]["service"]["temp"].as_str().expect("Error beim lesen des Service temp path");
            let service_temp_path = Path::new(service_temp_path_str);

            service_path.push("temp");
            if !service_path.exists(){
                fs::create_dir(&service_path).expect("Error beim erstellen des temp folders");
            }
        }
        //service/static
        {
            service_path.pop();
            service_path.push("static");
            if !service_path.exists() {
                fs::create_dir(&service_path).expect("Error beim erstellen des static folders");
            }
        }
    }


}
if !cloud_file_config_path.exists() {
        let url = "http://dev.phoenixcraft.eu/cloud/config.yml";
        let response = get(url)?.text()?;

        let mut file = File::create(&cloud_file_config_path)?;
        file.write_all(response.as_bytes())?;

        println!("Datei erstellt von {}", url);
    }




*/