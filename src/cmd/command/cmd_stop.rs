use crate::config::Config;
use crate::lib::thread_manager::ThreadManager;
use crate::sys_config::cloud_config::CloudConfig;
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
    ThreadManager::new().shutdown_all();
    println!("{} All Service Closed", Config::get_prefix());
}

fn remove_temp_folder() -> bool {
    let path = CloudConfig::get()
        .get_cloud_path()
        .get_service_folder()
        .get_temp_folder_path();
    match fs::remove_dir_all(&path) {
        Ok(_) => true,
        Err(_) => false,
    }
}
