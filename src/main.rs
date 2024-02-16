use crate::cloud::Cloud;

pub mod rest_api {
    pub mod api_main;
    pub mod get;
    pub mod set;
}

pub mod lib {
    pub mod address;
    pub mod bx;
    pub mod thread_manager;
    pub mod webhook;
}

pub mod utils {
    pub mod path;
    pub mod serde;
    pub mod service_status;
    pub mod log;
    pub mod logger;
    #[macro_use]
    pub mod logger_macros;
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

pub mod terminal {
    pub mod cmd;
    pub mod command_manager;
    pub mod command {
        pub mod cmd_help;
        pub mod cmd_service;
        pub mod cmd_task;
        pub mod cmd_template;
    }
}

pub mod cloud;
pub mod config;
pub mod language;
pub mod starting;

fn main() {
    println!("Start Game Cloud...");
    //start the game cloud
    Cloud::enable();

    //disable the game cloud
    Cloud::disable();

    println!("Game Cloud Stop");
    println!("Good Bye");
}
