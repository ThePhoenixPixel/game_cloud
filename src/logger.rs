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
    fn log(args: std::fmt::Arguments, log_level: Log) {
        // Erfasse die formatierten Argumente
        let formatted_msg = format_args!(
            "{} {} {} {}",
            Config::get_prefix(),
            Log::get(log_level).to_string(),
            ColoredString::from(">>").blue(),
            args
        );

        // Gib die formatierten Argumente auf der Konsole aus
        println!("{}", formatted_msg);

        // Schreibe die formatierten Argumente in die Log-Datei
        Logger::write_in_file(formatted_msg.to_string());
    }

    pub fn info(args: std::fmt::Arguments) {
        Logger::log(args, Log::Info);
    }

    pub fn warning(args: std::fmt::Arguments) {
        Logger::log(args, Log::Warning);
    }

    pub fn error(args: std::fmt::Arguments) {
        Logger::log(args, Log::Error);
    }

    fn write_in_file(msg: String) {
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

        if writeln!(file, "{}", msg).is_err() {
            eprintln!("Log System has an Error");
        }
    }
}
