use crate::cmd::cmd::Cmd;
use crate::config::Config;
use crate::logger::Logger;
use crate::starting::Starting;
use std::env;
use std::path::PathBuf;

pub mod language;
pub mod starting;
pub mod lib {
    pub mod address;
    pub mod bx;
    pub mod thread_manager;
}

pub mod utils {
    pub mod path;
    pub mod service_status;
}

pub mod cmd {
    pub mod cmd;
    pub mod command_manager;

    pub mod command {
        pub mod command_task;
        //pub mod cmd_group;
        //pub mod cmd_node;
        //pub mod cmd_software;
        pub mod cmd_stop;
        pub mod cmd_task;
        pub mod cmd_template;
    }
}
pub mod data {
    pub mod task;
    pub mod template;
    //pub mod group;
    //pub mod node;
    pub mod installer;
    pub mod service;
    pub mod software;
}

pub mod sys_config {
    pub mod cloud_config;
    pub mod software_config;
}
pub mod config;
pub mod log;
pub mod logger;

pub struct Main;

impl Main {
    pub fn get_exe_path() -> PathBuf {
        return match env::current_exe() {
            Ok(mut exe) => {
                exe.pop();
                exe
            }
            Err(e) => {
                Logger::error("Error get the exe path");
                Logger::error(e.to_string().as_str());
                panic!("The GameCloud has an fatal Error")
            }
        };
    }
}

fn main() {
    println!("Start Game Cloud...");

    let mut exe_path: PathBuf =
        env::current_exe().expect("Fehler beim Abrufen des Ausführungspfads");
    exe_path.pop();

    //start the cloud
    if Starting::start(exe_path) {
        Logger::info("Game Cloud start");

        let mut cmd = Cmd::new();
        cmd.set_prefix(Config::get_prefix());
        cmd.start();
        //end

        Logger::info(Config::get_prefix().to_string().as_str());
    }

    Logger::info("Game Cloud Stop");
    Logger::info("Good Bye");
}
