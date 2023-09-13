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
                //web config ändern
                "delete" => {
                    CmdTask::delete(args);
                }

                "setup" => {

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
            if  let Some(mut task) = Task::get_task(task_name.to_string()){
                if let Some(atribut) = args.get(2) {
                    match atribut.as_str() {
                        "name" => {
                            task.set_name(task_name.to_string());
                        }

                        "delete_on_stop" => {
                            if let Some(wert_new) = args.get(3) {
                                task.set_delete_on_stop(wert_new);
                            }
                        }

                        "static_service" => {
                            if let Some(wert_new) = args.get(3) {
                                task.set_static_service(wert_new);
                            }
                        }

                        "nodes" => {

                        }

                        "software" => {

                        }

                        "start_port" => {
                            if let Some(wert_new) = args.get(3) {
                                task.set_start_port(wert_new);
                            }
                        }

                        "min_service_count" => {
                            if let Some(wert_new) = args.get(3) {
                                task.set_min_service_count(wert_new);
                            }
                        }

                        "groups" => {

                        }

                        "templates" => {

                        }

                        _ => {
                            println!("{} Kein passendes Argument", Config::get_prefix());
                        }
                    }
                }
            }


        } else {
            println!("{} Please give a task name to change this", Config::get_prefix())
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