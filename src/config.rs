use std::{env, fs};
use std::path::PathBuf;
use colored::{ColoredString, Colorize};
use serde_json;

pub struct Config;


impl Config{

    pub fn get_system_plugins_path() -> PathBuf {
        let mut exe_path = env::current_exe().expect("Fehler beim Abrufen des Ausführungspfads");
        exe_path.pop();
        let config_path = exe_path.join("config.json");

        let config_content = fs::read_to_string(&config_path).expect("Fehler beim Lesen der Konfigurationsdatei");
        let config: serde_json::Value = serde_json::from_str(&config_content).expect("Fehler beim Deserialisieren der Konfiguration");

        let system_plugins_path = config["path"]["config"]["system_plugins"].as_str().expect("Pfad zur Aufgabe nicht gefunden");
        let system_plugins_path = exe_path.join(system_plugins_path);

        system_plugins_path
    }

    pub fn get_server_host() -> String {
        let mut exe_path = env::current_exe().expect("Error beim lesen des exe Path");
        exe_path.pop();
        let config_path = exe_path.join("config.json");

        let config_content = fs::read_to_string(&config_path).expect("Fehler beim Lesen der Konfigurationsdatei");
        let config: serde_json::Value = serde_json::from_str(&config_content).expect("Fehler beim Deserialisieren der Konfiguration");

        let server_host = config["server_host"].as_str().unwrap_or("127.0.0.1");
        server_host.to_string()
    }

    pub fn get_node_listener() -> String {
        let listener = format!("{}:{}", Config::get_node_host(), Config::get_node_port());
        listener
    }

    pub fn get_node_port() -> u64 {
        let mut exe_path = env::current_exe().expect("Error beim lesen des exe Path");
        exe_path.pop();
        let config_path = exe_path.join("config.json");

        let config_content = fs::read_to_string(&config_path).expect("Fehler beim Lesen der Konfigurationsdatei");
        let config: serde_json::Value = serde_json::from_str(&config_content).expect("Fehler beim Deserialisieren der Konfiguration");

        let node_port = config["node_port"].as_u64().unwrap_or(50263);
        node_port
    }

    pub fn get_node_host() -> String {
        let mut exe_path = env::current_exe().expect("Error beim lesen des exe Path");
        exe_path.pop();
        let config_path = exe_path.join("config.json");

        let config_content = fs::read_to_string(&config_path).expect("Fehler beim Lesen der Konfigurationsdatei");
        let config: serde_json::Value = serde_json::from_str(&config_content).expect("Fehler beim Deserialisieren der Konfiguration");

        let node_host = config["node_host"].to_string();
        node_host
    }

    pub fn get_prefix() -> ColoredString {
        let mut exe_path = env::current_exe().expect("Error beim lesen des exe Path");
        exe_path.pop();
        let config_path = exe_path.join("config.json");

        let config_content = fs::read_to_string(&config_path).expect("Fehler beim Lesen der Konfigurationsdatei");
        let config: serde_json::Value = serde_json::from_str(&config_content).expect("Fehler beim Deserialisieren der Konfiguration");

        let prefix = config["prefix"].as_str().unwrap_or("[Game Cloud]"); // Wenn kein Prefix gefunden wird, verwende "[Game Cloud]"
        prefix.bright_blue()
    }

    pub fn get_task_path() -> PathBuf {
        let mut exe_path = env::current_exe().expect("Fehler beim Abrufen des Ausführungspfads");
        exe_path.pop();
        let config_path = exe_path.join("config.json");

        let config_content = fs::read_to_string(&config_path).expect("Fehler beim Lesen der Konfigurationsdatei");
        let config: serde_json::Value = serde_json::from_str(&config_content).expect("Fehler beim Deserialisieren der Konfiguration");

        let task_relative_path = config["path"]["task"].as_str().expect("Pfad zur Aufgabe nicht gefunden");
        let task_path = exe_path.join(task_relative_path);

        task_path
    }

    pub fn get_template_path() -> PathBuf {
        let mut exe_path = env::current_exe().expect("Fehler beim Abrufen des Ausführungspfads");
        exe_path.pop();
        let config_path = exe_path.join("config.json");

        let config_content = fs::read_to_string(&config_path).expect("Fehler beim Lesen der Konfigurationsdatei");
        let config: serde_json::Value = serde_json::from_str(&config_content).expect("Fehler beim Deserialisieren der Konfiguration");

        let template_relative_path = config["path"]["template"].as_str().expect("Pfad zur Aufgabe nicht gefunden");
        let template_path = exe_path.join(template_relative_path);

        template_path
    }

    pub fn get_service_temp_path() -> PathBuf {
        let mut exe_path = env::current_exe().expect("Fehler beim Abrufen des Ausführungspfads");
        exe_path.pop();
        let config_path = exe_path.join("config.json");

        let config_content = fs::read_to_string(&config_path).expect("Fehler beim Lesen der Konfigurationsdatei");
        let config: serde_json::Value = serde_json::from_str(&config_content).expect("Fehler beim Deserialisieren der Konfiguration");

        let service_temp_relative_path = config["path"]["service"]["temp"].as_str().expect("Pfad zur Aufgabe nicht gefunden");
        let service_temp_path = exe_path.join(service_temp_relative_path);

        service_temp_path
    }

    pub fn get_service_static_path() -> PathBuf {
        let mut exe_path = env::current_exe().expect("Fehler beim Abrufen des Ausführungspfads");
        exe_path.pop();
        let config_path = exe_path.join("config.json");

        let config_content = fs::read_to_string(&config_path).expect("Fehler beim Lesen der Konfigurationsdatei");
        let config: serde_json::Value = serde_json::from_str(&config_content).expect("Fehler beim Deserialisieren der Konfiguration");

        let service_static_relative_path = config["path"]["service"]["static"].as_str().expect("Pfad zur Aufgabe nicht gefunden");
        let service_static_path = exe_path.join(service_static_relative_path);

        service_static_path
    }

    pub fn get_config_links_path() -> PathBuf{
        let mut exe_path = env::current_exe().expect("Fehler beim Abrufen des Ausführungspfads");
        exe_path.pop();
        let config_path = exe_path.join("config.json");

        let config_content = fs::read_to_string(&config_path).expect("Fehler beim Lesen der Konfigurationsdatei");
        let config: serde_json::Value = serde_json::from_str(&config_content).expect("Fehler beim Deserialisieren der Konfiguration");

        let config_links_relative_path = config["path"]["config"]["links"].as_str().expect("Pfad zur Aufgabe nicht gefunden");
        let mut config_links_path :PathBuf= exe_path.join(config_links_relative_path);
        config_links_path.push("links.json");
        config_links_path
    }

    pub fn get_config_default_task_path() -> PathBuf {
        let mut exe_path = env::current_exe().expect("Fehler beim Abrufen des Ausführungspfads");
        exe_path.pop();
        let config_path = exe_path.join("config.json");

        let config_content = fs::read_to_string(&config_path).expect("Fehler beim Lesen der Konfigurationsdatei");
        let config: serde_json::Value = serde_json::from_str(&config_content).expect("Fehler beim Deserialisieren der Konfiguration");

        let config_default_task_relative_path = config["path"]["config"]["default_task"].as_str().expect("Pfad zur Aufgabe nicht gefunden");
        let mut config_default_task_path = exe_path.join(config_default_task_relative_path);
        config_default_task_path.push("task.json");
        config_default_task_path
    }

    pub fn get_software_path() -> PathBuf {
        let mut exe_path = env::current_exe().expect("Fehler beim Abrufen des Ausführungspfads");
        exe_path.pop();
        let config_path = exe_path.join("config.json");

        let config_content = fs::read_to_string(&config_path).expect("Fehler beim Lesen der Konfigurationsdatei");
        let config: serde_json::Value = serde_json::from_str(&config_content).expect("Fehler beim Deserialisieren der Konfiguration");

        let config_default_task_relative_path = config["path"]["config"]["software"].as_str().expect("Pfad zur Aufgabe nicht gefunden");
        let mut software_path = exe_path.join(config_default_task_relative_path);
        software_path.push("software.json");
        software_path
    }

    pub fn get_software_files_path() -> PathBuf {
        let mut exe_path = env::current_exe().expect("Error beim lesen des Exe Path");
        exe_path.pop();
        let config_path = exe_path.join("config.json");

        let config_content = fs::read_to_string(&config_path).expect("Fehler beim Lesen der Konfigurationsdatei");
        let config: serde_json::Value = serde_json::from_str(&config_content).expect("Fehler beim Deserialisieren der Konfiguration");

        let config_default_software_files_relative_path = config["path"]["config"]["software_files"].as_str().expect("software files Path kann nicht gelsesen werden");
        let mut software_files_path = exe_path.join(config_default_software_files_relative_path);
        software_files_path
    }

}