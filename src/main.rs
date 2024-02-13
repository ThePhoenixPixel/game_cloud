use crate::cloud::Cloud;

pub mod language;
pub mod starting;

pub mod rest_api {
    pub mod api_main;
    pub mod get;
    pub mod set;
}
pub mod lib {
    pub mod address;
    pub mod bx;
    pub mod webhook;
    pub mod thread_manager;
}

pub mod utils {
    pub mod path;
    pub mod serde;
    pub mod service_status;
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
pub mod cloud;
pub mod config;
pub mod log;
pub mod logger;

fn main() {
    println!("Start Game Cloud...");
    //start the game cloud
    Cloud::enable();

    //disable the game cloud
    Cloud::disable();

    println!("Game Cloud Stop");
    println!("Good Bye");
}
