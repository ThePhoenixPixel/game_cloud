use std::sync::atomic::compiler_fence;
use crate::config::Config;
use crate::data::task;
use crate::data::task::Task;

pub struct CmdTask;


impl CmdTask{
    pub fn execute(args: &Vec<String>){
        if let  Some(arg1) = args.get(1) {
            match arg1 {

                &String::from("create") => {

                }

                &String::from("info") => {
                    CmdTask::info(&args)
                }

                &String::from("delete") => {

                }

                _ => {
                    eprintln!("{} Kein gueltiges Argument", Config::get_prefix());
                }
            }
        } else {
            eprintln!("{} Bitte gebe ein Argument an", Config::get_prefix());
        }
    }

    fn info(args: &Vec<String>) {
        if let  Some(arg2) = args.get(2) {
            let task = Task::get_task(arg2.to_string()).unwrap();


        }
    }

}