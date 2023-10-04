use crate::config::Config;
use crate::data::software::Software;
use crate::data::task::Task;
use crate::data::template::Template;

pub struct CmdTask;


impl CmdTask{
    pub fn execute(args: &Vec<String>){

        if let Some(arg0) = args.get(0) {

            match arg0.as_str() {

                "create" => {
                    CmdTask::create(args);
                }

                "info" => {
                    CmdTask::info(args);
                }

                "delete" => {
                    CmdTask::delete(args);

                }

                "setup" => {
                    CmdTask::setup(args);

                }

                "reload" => {
                    Task::reload();
                }

                _ => {
                    println!("{}", arg0);
                    eprintln!("{} Kein gueltiges Argument", Config::get_prefix());
                }
            }
        } else {
            eprintln!("{} Bitte gebe ein Argument an", Config::get_prefix());
        }
    }

    fn setup(args: &Vec<String>) {
        if args.get(1).is_some() {
            if let  Some(arg) = args.get(2) {
                match arg.to_lowercase().as_str() {
                    "add" => {
                        CmdTask::setup_add(args);
                    }

                    "set" => {
                        CmdTask::setup_set(args);
                    }

                    "remove" => {

                    }

                    "clear" => {

                    }

                    &_ => {
                        println!("{} Kein Gültiges Argument", Config::get_prefix());
                    }
                }
            } else {
                println!("{} Please give set/add/remove/clear", Config::get_prefix());
            }
        } else {
            println!("{} Please give a task name to change this", Config::get_prefix())
        }
    }

    fn setup_remove(args: &Vec<String>){

        let task_name = args.get(1).unwrap().to_string();
        //let attribut = args.get(3).unwrap().to_lowercase();
        //let new_wert = args.get(4);

        match Task::get_task(task_name.clone()){
            Some(t) => t,
            None => {
                println!("{} Task '{}' nicht gefunden.", Config::get_prefix(), task_name);
                return;
            }
        };



    }

    //task setup <name> set <attribut> <new wert>
    fn setup_set(args: &Vec<String>) {
        if args.len() < 5 {
            println!("{} Bitte geben Sie mindestens 5 Argumente an.", Config::get_prefix());
            return;
        }

        let task_name = args.get(1).unwrap();
        let attribut = args.get(3).unwrap().to_lowercase();
        let new_wert = args.get(4);

        let mut task = match Task::get_task(task_name.clone()) {
            Some(t) => t,
            None => {
                println!("{} Task '{}' nicht gefunden.", Config::get_prefix(), task_name);
                return;
            }
        };

        match attribut.as_str() {
            "name" => {
                if let Some(new_name) = new_wert {
                    task.change_name(new_name.clone());
                    println!("{} Setze den Task-Namen auf '{}'.", Config::get_prefix(), new_name);
                } else {
                    println!("{} Bitte geben Sie einen neuen Namen an.", Config::get_prefix());
                }
            }

            "delete_on_stop" => {
                if let Some(new_value) = new_wert {
                    match new_value.to_lowercase().as_str() {
                        "true" => task.set_delete_on_stop(true),
                        "false" => task.set_delete_on_stop(false),
                        _ => {
                            println!("{} Ungültiger Wert für '{}': {}. Verwenden Sie 'true' oder 'false'.", Config::get_prefix(), attribut, new_value);
                            return;
                        }
                    }

                    println!("{} Setze '{}' auf '{}'.", Config::get_prefix(), attribut, new_value);
                } else {
                    println!("{} Bitte geben Sie einen neuen Wert für '{}' an.", Config::get_prefix(), attribut);
                }
            }

            "static_service" => {
                if let Some(new_value) = new_wert {
                    match new_value.to_lowercase().as_str() {
                        "true" => task.set_static_service(true),
                        "false" => task.set_static_service(false),
                        _ => {
                            println!("{} Ungültiger Wert für '{}': {}. Verwenden Sie 'true' oder 'false'.", Config::get_prefix(), attribut, new_value);
                            return;
                        }
                    }

                    println!("{} Setze '{}' auf '{}'.", Config::get_prefix(), attribut, new_value);
                } else {
                    println!("{} Bitte geben Sie einen neuen Wert für '{}' an.", Config::get_prefix(), attribut);
                }
            }

            "software" => {
                if args.len() < 6 {
                    println!("{} Bitte geben Sie den Typ und den Namen der Software an.", Config::get_prefix());
                    return;
                }

                let software_type = args.get(5).unwrap();
                if let Some(software_name) = new_wert {
                    let mut software = Software::new();
                    software.set_software_type(&software_type);
                    software.set_name(&software_name);
                    task.set_software(software);
                    println!("{} Setze 'Software' auf '{} {}'", Config::get_prefix(), software_type, software_name);
                } else {
                    println!("{} Bitte geben Sie den Namen der Software an.", Config::get_prefix());
                }
            }

            "start_port" => {
                if let Some(start_port_str) = new_wert {
                    match start_port_str.parse::<u32>() {
                        Ok(start_port) => {
                            task.set_start_port(start_port);
                            println!("{} Setze den Start-Port auf {}.", Config::get_prefix(), start_port);
                        }
                        Err(_) => {
                            println!("{} Ungültiger Wert für den Start-Port: {}", Config::get_prefix(), start_port_str);
                        }
                    }
                } else {
                    println!("{} Bitte geben Sie einen Wert für den Start-Port an.", Config::get_prefix());
                }
            }

            "min_service_count" => {
                if let Some(min_service_count_str) = new_wert {
                    match min_service_count_str.parse::<u32>() {
                        Ok(min_service_count) => {
                            task.set_min_service_count(min_service_count);
                            println!("{} Setze 'Min Service Count' auf {}.", Config::get_prefix(), min_service_count);
                        }
                        Err(_) => {
                            println!("{} Ungültiger Wert für 'Min Service Count': {}", Config::get_prefix(), min_service_count_str);
                        }
                    }
                } else {
                    println!("{} Bitte geben Sie einen Wert für 'Min Service Count' an.", Config::get_prefix());
                }
            }

            _ => {
                println!("{} Ungültiges Attribut. Bitte geben Sie 'name', 'delete_on_stop', 'static_service', 'software', 'start_port' oder 'min_service_count' an.", Config::get_prefix());
            }
        }
    }


    //task setup <name> add <attribut> <new wert>
    fn setup_add(args: &Vec<String>) {
        if let Some(attribut) = args.get(3) {
            if let Some(new_wert) = args.get(4) {

                let mut task = Task::get_task(args.get(1).unwrap().to_string()).unwrap();

                match attribut.to_lowercase().as_str() {
                    "group" => {
                        task.add_group(&new_wert.to_string());
                        println!("{} Added group {} to the Task", Config::get_prefix(), new_wert);
                    }

                    "node" => {
                        task.add_node(new_wert.to_string());
                        println!("{} Added node {} to the task", Config::get_prefix(), new_wert);
                    }

                    "template" => {
                        println!("This feature is not implemented yet.");
                    }

                    _ => {
                        println!("{} Please specify 'group', 'node', or 'template'", Config::get_prefix());
                    }
                }
            } else {
                println!("{} Please provide a value to add", Config::get_prefix());
            }
        } else {
            println!("{} Please specify an attribute to change", Config::get_prefix());
        }
    }



    fn create(args: &Vec<String>){
        //check task name is set
        if let Some(task_name) = args.get(1) {

            //check ob task exits
            if Task::is_exist(task_name.to_string()) {
                println!("{} task {} allready exist", Config::get_prefix().to_string(), task_name);
                return;
            }

            if let Some(software_type) = args.get(2) {

                if let  Some(software_name) = args.get(3) {

                    create_task(task_name.to_string(), software_type.to_string(), software_name.to_string());

                } else {
                    //hannes hat die Zeile geschrieben
                    println!("{} bitte gebe ein software name ein", Config::get_prefix());
                }

            } else {
                println!("{} Bitte gebe ein Software Type ein", Config::get_prefix());
            }

        } else {
            println!("{} Bitte gebe ein namen an", Config::get_prefix());
            println!("{} task create <name> <Server_Type> <Software>", Config::get_prefix());
        }
    }

    fn delete(args: &Vec<String>){

        if let  Some(task_name) = args.get(1) {
            if let Some(task) = Task::get_task(task_name.to_string()) {

                task.delete_as_file();
                println!("{} Sucessful delete the task {}", Config::get_prefix(), task_name);

            } else {
                println!("{} Task does not exist", Config::get_prefix());
            }

        } else {
            println!("{} Please give a task name", Config::get_prefix())
        }

    }

    fn info(args: &Vec<String>) {
        if let  Some(arg1) = args.get(1) {
            if let Some(task) = Task::get_task(arg1.to_string()){
                //print task
                println!("{} | Type             | Wert", Config::get_prefix());
                println!("{} | ----------------------------------------------", Config::get_prefix());
                println!("{} | Name             | {}", Config::get_prefix(), task.get_name());
                println!("{} | Delete On Stop   | {}", Config::get_prefix(), task.get_delete_on_stop());
                println!("{} | Static Service   | {}", Config::get_prefix(), task.get_static_service());
                println!("{} | Nodes            | {:?}", Config::get_prefix(), task.get_nodes());
                //print Software
                println!("{} | Software:        | ", Config::get_prefix());
                println!("{} |      Type        | {}", Config::get_prefix(), task.get_software().get_software_type());
                println!("{} |      Name        | {}", Config::get_prefix(), task.get_software().get_name());
                println!("{} |      Max Ram     | {}", Config::get_prefix(), task.get_software().get_max_ram());
                //print Port
                println!("{} | Start Port       | {}", Config::get_prefix(), task.get_start_port());
                println!("{} | MinServiceCount  | {}", Config::get_prefix(), task.get_min_service_count());
                println!("{} | Groups           | {:?}", Config::get_prefix(), task.get_groups());
                //print Template
                println!("{} | Templates:       | ", Config::get_prefix());

            } else {
                //task not exsits
                println!("{} Task does not exsists", Config::get_prefix());
            }
        }
    }
}


//create fn for default task objekt
fn create_task(name: String, software_type: String, software_name: String){
    let mut software = Software::new();
    software.set_software_type(&software_type);
    software.set_name(&software_name);

    if Software::get_software_url(&software).is_some() {

    } else {
        println!("{} Software nicht gefunden oder ungültig", Config::get_prefix());
        println!("{} Bitte geben sie eine forhandene Software an oder fügen sie eine hinzu", Config::get_prefix());
        return;
    }

    //cerate task and software objekkts
    let mut task = Task::new();
    let mut software = Software::new();
    let mut template = Template::new();

    //steupp template
    template.set_template(&name);
    template.set_name(&"default".to_string());
    let priority:u32 = 1;
    template.set_priority(&priority);

    //setup software with parameters
    software.set_software_type(&software_type);
    software.set_name(&software_name);

    //setup the task objekt
    task.set_software(software);
    task.change_name(name);
    task.clear_templates();
    task.add_template(template);

    //save the new task to a file
    task.save_to_file();
}