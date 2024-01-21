use crate::config::Config;
use crate::lib::bx::Bx;
use crate::log::Log;
use chrono::Local;
use colored::{ColoredString, Colorize};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

pub struct Logger;

impl Logger {
    pub fn info(msg: &str) {
        println!(
            "{} {} {}",
            Config::get_prefix(),
            Log::get(Log::Info),
            ColoredString::from(msg).green()
        );
        Logger::write_in_file(msg, Log::Info);
    }

    pub fn warning(msg: &str) {
        println!(
            "{} {} {}",
            Config::get_prefix(),
            Log::get(Log::Warning),
            ColoredString::from(msg).yellow()
        );
        Logger::write_in_file(msg, Log::Warning);
    }

    pub fn error(msg: &str) {
        println!(
            "{} {} {}",
            Config::get_prefix(),
            Log::get(Log::Error),
            ColoredString::from(msg).red()
        );
        Logger::write_in_file(msg, Log::Error);
    }

    fn write_in_file(msg: &str, log: Log) {
        let mut log_path =
            env::current_exe().expect("Cloud Error can not get the exe path of the cloud system");
        log_path.pop();
        log_path.push("log");
        Bx::create_path(&log_path);

        let file_name = format!("log_{}.log", Local::now().format("%Y-%m-%d"));
        log_path.push(&file_name);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .expect("Log system has an error and cannot create the log file");

        if write!(
            file,
            "{} {} {} {}\n",
            Config::get_prefix().to_string(),
            Log::get(log).to_string(),
            ColoredString::from(">>").blue(),
            msg
        )
        .is_err()
        {
            eprintln!("Log System has an Error");
        }
    }
}
