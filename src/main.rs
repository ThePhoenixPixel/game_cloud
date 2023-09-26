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

    println!("Start Game Cloud...");

    let mut exe_path:PathBuf = env::current_exe().expect("Fehler beim Abrufen des Ausführungspfads");
    exe_path.pop();


    //start the cloud
    if Starting::start(exe_path){

        let mut cmd = Cmd::new();
        cmd.set_prefix(Config::get_prefix());
        cmd.start();
    }


    //end
    println!("{} BB", Config::get_prefix());
}
