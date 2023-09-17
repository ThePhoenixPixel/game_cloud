use crate::config::Config;
use crate::data::software::Software;
use crate::data::task::Task;

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

                _ => {
                    println!("{}", arg0);
                    eprintln!("{} Kein gueltiges Argument", Config::get_prefix());
                }
            }
        } else {
            eprintln!("{} Bitte gebe ein Argument an", Config::get_prefix());
        }
    }

    fn setup(args: &Vec<String>){
        if let Some(task_name) = args.get(1) {
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

    //task setup <name> set <attribut> <new wert>
    fn setup_set(args: &Vec<String>){
        if let Some(attribut) = args.get(3) {
            if let Some(new_wert) = args.get(4).unwrap().to_string(){

                let mut task = (Task::get_task(args.get(1).unwrap().to_string())).unwrap();

                match attribut.to_lowercase().as_str() {

                    "name" => {
                        task.set_name(new_wert);
                        println!("{} Set Task Name to {}", Config::get_prefix(), new_wert);
                    }

                    "delete_on_stop" => {
                        task.set_delete_on_stop(new_wert);
                        println!("{} Set deleteOnStop to {}", Config::get_prefix(), new_wert);
                    }

                    "static_service" => {
                        task.set_static_service(new_wert);
                        println!("{} Set Static Service to {}", Config::get_prefix(), new_wert);
                    }

                    "software" => {
                        if let Some(software_name) = args.get(5) {
                            let mut software = Software::new();
                            software.set_name(software_name.to_string());
                            task.set_software(software);
                            println!("{} Set Software to {} {}", Config::get_prefix(), new_wert, software_name);
                        }


                    }

                    "start_port" => {

                    }

                    "min_service_count" => {

                    }

                    &_ => {
                        println!("{} Pleas give name/delete_on_stop/static_service/", Config::get_prefix());
                    }
                }
            } else {
                println!("{} Pleas give a ", Config::get_prefix());
            }
        } else {
            println!("{} Please give a attribut to chang this", Config::get_prefix());
        }
    }

    //task setup <name> add <attribut> <new wert>
    fn setup_add(args: &Vec<String>){
        if let Some(attribut) = args.get(3) {
            if let Some(new_wert) = args.get(4).unwrap().to_string() {

                let mut task = (Task::get_task(args.get(1).unwrap().to_string()).unwrap());

                match attribut.to_lowercase().as_str() {
                    "group" => {
                        task.add_group(new_wert);
                        println!("{} Add node {} to the Task", Config::get_prefix(), new_wert)
                    }

                    "node" => {
                        task.add_node(new_wert);
                        println!("{} Add node {} to the task", Config::get_prefix(), new_wert);
                    }

                    "template" => {
                        println!("Dies geht noch nicht");
                    }

                    &_ => {
                        println!("{} Plaase give group/node/template", Config::get_prefix());
                    }
                }
            } else {
                println!("{} Pleas give a ", Config::get_prefix());
            }
        } else {
            println!("{} Please give a attribut to chang this", Config::get_prefix());
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
    //cerate task and software objekkts
    let mut task = Task::new();
    let mut software = Software::new();

    //setup software with parameters
    software.set_software_type(software_type);
    software.set_name(software_name);
    
    //setup the task objekt
    task.set_software(software);
    task.set_name(name);
    
    //save the new task to a file
    task.save_to_file();
}