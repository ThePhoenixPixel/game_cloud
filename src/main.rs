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
