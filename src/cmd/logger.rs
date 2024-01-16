use std::fs::OpenOptions;
use colored::{ColoredString, Colorize};
use crate::config::Config;
use std::io::Write;
use crate::cmd::log::Log;

pub struct Logger;

impl Logger {

    pub fn info(msg: &str) {
        println!("{} {} {}", Config::get_prefix(), Log::get(Log::Info), ColoredString::from(msg).green());
        Logger::write_in_file(msg, Log::Info);
    }

    pub fn warning(msg: &str) {
        println!("{} {} {}", Config::get_prefix(), Log::get(Log::Warning), ColoredString::from(msg).yellow());
        Logger::write_in_file(msg, Log::Warning);
    }

    pub fn error(msg: &str) {
        println!("{} {} {}", Config::get_prefix(), Log::get(Log::Error), ColoredString::from(msg).red());
        Logger::write_in_file(msg, Log::Error);
    }

    fn write_in_file(msg: &str, log: Log) {
        let mut file = OpenOptions::new().create(true).append(true).open("log_file.log").expect("Log system hast an error cannot be create the log file");
        if write!(file, "{} {} {} \n", Config::get_prefix(), Log::get(log), msg).is_err() {
            eprintln!("Log System has an Error");
        }
    }
}

