use crate::cmd::cmd::Cmd;
use crate::cmd::logger::Logger;
use crate::config::Config;
use crate::starting::Starting;
use crate::sys_config::software_config::SoftwareConfig;
use std::env;
use std::path::PathBuf;

pub mod language;
pub mod starting;
pub mod lib {
    pub mod address;
    pub mod bx;
}

pub mod utils {
    pub mod path;
    pub mod service_status;
}

pub mod cmd {
    pub mod cmd;
    pub mod command_manager;
    pub mod log;
    pub mod logger;
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
    pub mod software_config;
}
pub mod config;

fn main() {
    println!("Start Game Cloud...");

    let mut exe_path: PathBuf =
        env::current_exe().expect("Fehler beim Abrufen des Ausführungspfads");
    exe_path.pop();

    //start the cloud
    if Starting::start(exe_path) {
        Logger::info("das ist eine kleine info");
        Logger::warning("das ist eine kleine warning");
        Logger::error("das ist eine kleine error");
        let mut cmd = Cmd::new();
        cmd.set_prefix(Config::get_prefix());
        cmd.start();
        //end

        println!("{} BB", Config::get_prefix());
    }

    println!("Game Cloud Stop");
    println!("Good Bye");
}
