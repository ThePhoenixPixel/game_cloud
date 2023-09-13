use crate::config::Config;
use crate::data::task::Task;

pub struct CmdTask;


impl CmdTask{
    pub fn execute(args: &Vec<String>){

        if let Some(arg1) = args.get(0) {
            match arg1.as_str() {

                "create" => {

                }

                "info" => {

                    CmdTask::info(args);
                }

                "delete" => {

                }

                _ => {
                    println!("{}", arg1);
                    eprintln!("{} Kein gueltiges Argument", Config::get_prefix());
                }
            }
        } else {
            eprintln!("{} Bitte gebe ein Argument an", Config::get_prefix());
        }
    }

    fn info(args: &Vec<String>) {
        if let  Some(arg2) = args.get(1) {
            if let Some(task) = Task::get_task(arg2.to_string()){
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