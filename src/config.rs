use crate::lib::address::Address;
use crate::sys_config::cloud_config::CloudConfig;
use colored::{ColoredString, Colorize};
use serde_json;
use std::path::PathBuf;
use std::{env, fs};

pub struct Config;

impl Config {
    pub fn get_prefix() -> ColoredString {
        let mut exe_path = env::current_exe().expect("Error beim lesen des exe Path");
        exe_path.pop();
        let config_path = exe_path.join("config.json");

        let config_content =
            fs::read_to_string(&config_path).expect("Fehler beim Lesen der Konfigurationsdatei");
        let config: serde_json::Value = serde_json::from_str(&config_content)
            .expect("Fehler beim Deserialisieren der Konfiguration");

        let prefix = config["prefix"].as_str().unwrap_or("[Game Cloud]"); // Wenn kein Prefix gefunden wird, verwende "[Game Cloud]"
        prefix.bright_blue()
    }

    pub fn get_language_path() -> PathBuf {
        let mut exe_path = env::current_exe().expect("Error beim lesen des Exe Path");
        exe_path.pop();
        let config_path = exe_path.join("config.json");

        let config_content =
            fs::read_to_string(&config_path).expect("Fehler beim Lesen der Konfigurationsdatei");
        let config: serde_json::Value = serde_json::from_str(&config_content)
            .expect("Fehler beim Deserialisieren der Konfiguration");

        let config_default_software_files_relative_path = config["language"]
            .as_str()
            .expect("software files Path kann nicht gelsesen werden");
        let software_files_path = exe_path.join(config_default_software_files_relative_path);
        software_files_path
    }
}
