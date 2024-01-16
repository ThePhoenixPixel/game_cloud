use crate::config::Config;
use std::fs;

pub struct CmdStop;

impl CmdStop {
    pub fn execute(_args: &Vec<String>) -> bool {
        shutdown_all_service();
        remove_temp_folder();
        true
    }
}

fn shutdown_all_service() {
    println!("{} All Service Closed", Config::get_prefix());
}

fn remove_temp_folder() -> bool {
    let path = Config::get_service_temp_path();
    match fs::remove_dir_all(&path) {
        Ok(_) => true,
        Err(_) => false,
    }
}
