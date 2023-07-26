use std::fs;
use std::io;
use std::env;
use std::path::{PathBuf};
use crate::data::Task::Task;

mod data{
    pub mod Task;
}

fn main(){
    println!("Start Game Cloud");
    let dev_mode = false;
    //read file and dirs
    let mut exe_path:PathBuf = env::current_exe().expect("Fehler beim Abrufen des Ausführungspfads");
    exe_path.pop();
    check_dir(exe_path);



    //task abfrage und initaliesirung der
    let task_all = Task::get_task_all();

    println!("{:?}", task_all);

    for task_name in task_all {
        let task = Task::get_task(task_name.as_str()).expect("Fehler beim Abrufen der Aufgabe");

        if dev_mode {
            println!("-------------------------------------------");
            println!("Name: {}", task.get_name());
            println!("Mindestanzahl Dienste: {}", task.get_minservicecount());
            println!("Maximaler RAM: {}", task.get_maxram());
            println!("Vorlage: {}", task.get_template());
            println!("-------------------------------------------");
        }

        if task.get_minservicecount() > 0 {
            for _ in 0..task.get_minservicecount() {
                println!("Dienst starten {}", &task.name);
                task.start_as_service();
            }
        }
    }


    // cmd
    loop{
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error beim lesen der eingabe");


        if !user_input(&input.trim().split_whitespace().collect::<Vec<&str>>()) {
            break;
        }
    }


    //end
    println!("BB");
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
                                        &"proxy" => {
                                            println!("task create {} proxy", sub1);
                                            println!("task erfolgreich erstellt (proxy)");

                                            let task = Task::new(sub1, 0, 1028, sub1);

                                            task.create();

                                        }
                                        &"server" => {
                                            println!("task create {} server", sub1);
                                            println!("task erfolgreich erstellt (Servertask )");

                                            let task = Task::new(sub1, 0, 1028, sub1);

                                            task.create();

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
                        &"delete" => {
                            println!("task delete");
                            if let Some(sub1) = args.get(2) {

                            } else{
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

fn shutdown(){

}

fn check_dir(exe_path: PathBuf){

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
/*if !cloud_file_config_path.exists() {
        let url = "http://dev.phoenixcraft.eu/cloud/config.yml";
        let response = get(url)?.text()?;

        let mut file = File::create(&cloud_file_config_path)?;
        file.write_all(response.as_bytes())?;

        println!("Datei erstellt von {}", url);
    }*/