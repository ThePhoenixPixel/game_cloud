use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use colored::*;
use reqwest::blocking::get;
use serde_json::{Map, Value};
use crate::config::Config;
use crate::data::task::Task;
use crate::lib::bx::Bx;

pub struct Starting;

impl Starting {
    pub fn start(exe_path: PathBuf) -> bool {
        Starting::print_icon();

        if let Some(config) = Starting::check_config(&exe_path) {
            let cmd_prefix = Config::get_prefix();
            Starting::check_folder(&exe_path, &config, &cmd_prefix);

            if Starting::check_link(&exe_path, &config, &cmd_prefix) {
                Starting::check_software(&exe_path, &cmd_prefix);

                Starting::check_task();
            } else {
                return false;
            }
        } else {
            return false;
        }
        return true;
    }

    fn check_task() {
        Task::reload();
    }


    fn print_icon() {
        println!(" ");
        println!("_____{}__________________________________________________________{}__{}________________________________________{}_____", r"/\\\\\\\\\\\\".red(), r"/\\\\\\\\\".cyan(), r"/\\\\\\".cyan(), r"/\\\".cyan() );
        println!("___{}________________________________________________________{}__{}_______________________________________{}_____", r"/\\\//////////".red(), r"/\\\////////".cyan(), r"\////\\\".cyan(), r"\/\\\".cyan() );
        println!("__{}_________________________________________________________________{}______________{}_______________________________________{}_____", r"/\\\".red(), r"/\\\/".cyan(), r"\/\\\".cyan(), r"\/\\\".cyan() );
        println!("_{}____{}__{}_______{}__{}_______{}___{}________________{}________{}_____{}____{}________{}_____", r"\/\\\".red(), r"/\\\\\\\".red(), r"/\\\\\\\\\".red(), r"/\\\\\".red(), r"/\\\\\".red(), r"/\\\\\\\\".red(), r"/\\\".cyan(), r"\/\\\".cyan(), r"/\\\\\".cyan(), r"/\\\".cyan(), r"/\\\".cyan(), r"\/\\\".cyan());
        println!("_{}___{}_{}____{}___{}_{}________________{}______{}__{}___{}___{}_____", r"\/\\\".red(), r"\/////\\\".red(), r"\////////\\\".red(), r"/\\\///\\\\\///\\\".red(), r"/\\\/////\\\".red(), r"\/\\\".cyan(), r"\/\\\".cyan(), r"/\\\///\\\".cyan(), r"\/\\\".cyan(), r"\/\\\".cyan(), r"/\\\\\\\\\".cyan());
        println!("__{}_______{}___{}__{}_{}__{}__{}__{}_______________{}_____{}__{}_{}___{}__{}____", r"\/\\\".red(), r"\/\\\".red(), r"/\\\\\\\\\\".red(), r"\/\\\".red(), r"\//\\\".red(), r"\/\\\".red(), r"/\\\\\\\\\\\".red(), r"\//\\\".cyan(), r"\/\\\".cyan(), r"/\\\".cyan(), r"\//\\\".cyan(), r"\/\\\".cyan(), r"\/\\\".cyan(), r"/\\\////\\\".cyan());
        println!("___{}_______{}__{}__{}__{}__{}_{}____{}_____________{}____{}__{}__{}___{}_{}__{}___", r"\/\\\".red(), r"\/\\\".red(), r"/\\\/////\\\".red(), r"\/\\\".red(), r"\/\\\".red(), r"\/\\\".red(), r"\//\\///////".red(), r"\///\\\".cyan(), r"\/\\\".cyan(), r"\//\\\".cyan(), r"/\\\".cyan(), r"\/\\\".cyan(), r"\/\\\".cyan(), r"\/\\\".cyan(), r"\/\\\".cyan());
        println!("____{}__{}_{}__{}__{}__{}____{}__{}__{}___{}__{}_", r"\//\\\\\\\\\\\\/".red(), r"\//\\\\\\\\/\\".red(), r"\/\\\".red(), r"\/\\\".red(), r"\/\\\".red(), r"\//\\\\\\\\\\".red(), r"\////\\\\\\\\\".cyan(), r"/\\\\\\\\\".cyan(), r"\///\\\\\/".cyan(), r"\//\\\\\\\\\".cyan(), r"\//\\\\\\\/\\".cyan());
        println!("_____{}_____{}__{}___{}___{}____{}________{}__{}_____{}______{}____{}__", r"\////////////".red(), r"\////////\//".red(), r"\///".red(), r"\///".red(), r"\///".red(), r"\//////////".red(), r"\/////////".cyan(), r"\/////////".cyan(), r"\/////".cyan(), r"\/////////".cyan(), r"\///////\//".cyan());
        println!(" ");
    }

    fn check_software(exe_path: &PathBuf, cmd_prefix: &ColoredString) {
        // Pfade und Verzeichnisse einrichten
        let install_dir = Config::get_software_files_path();

        // JSON-Datei mit Software-Links öffnen
        let software_path = Config::get_software_path();

        let mut software_file = File::open(&software_path).expect("Fehler beim Öffnen der Datei");

        let mut json_str = String::new();
        software_file
            .read_to_string(&mut json_str)
            .expect("Fehler beim Lesen der Datei");

        // JSON-String in ein JSON-Objekt (serde_json::Value) parsen
        let json_value: Value =
            serde_json::from_str(&json_str).expect("Fehler beim Deserialisieren des JSON-Strings");

        // Überprüfen, ob es sich um ein JSON-Objekt (eine Map) handelt
        if let Some(software_categories) = json_value.get("software").and_then(|s| s.as_object()) {
            // Iteriere durch die Software-Kategorien (server, proxy, self, usw.)
            for (software_type, software_links) in software_categories {
                // Überprüfe, ob es sich um ein JSON-Objekt (eine Map) mit Software-Links handelt
                if let Some(links) = software_links.as_object() {
                    // Iteriere durch die Links in dieser Kategorie (server, proxy, self, usw.)
                    for (software_name, software_link_value) in links {
                        // Überprüfe, ob der Link ein String ist
                        if let Some(software_link) = software_link_value.as_str() {
                            // Jetzt kannst du die Software installieren oder andere Aktionen durchführen
                            let software_dir = install_dir.join(software_type);

                            // Überprüfen und Verzeichnisse erstellen
                            if !software_dir.exists() {
                                fs::create_dir_all(&software_dir)
                                    .expect("Fehler beim Erstellen des Verzeichnisses");
                            }

                            install_software(software_link, software_name, software_type,cmd_prefix);
                        } else {
                            println!(
                                "{} Ungültiger Link für Software: {}",
                                cmd_prefix, software_name
                            );
                        }
                    }
                } else {
                    println!(
                        "{} Ungültige Links für Software-Typ: {}",
                        cmd_prefix, software_type
                    );
                }
            }
        } else {
            println!("{} Die JSON-Datei enthält keine 'software'-Kategorie", Config::get_prefix());
        }
    }

    fn check_config(exe_path: &PathBuf) -> Option<Value> {
        // config.json
        let mut config_file_path = exe_path.clone();
        config_file_path.push("config.json");

        if !config_file_path.exists() {
            let url = "http://37.114.62.121/cloud/default_file/config.json";
            if let Ok(response) = get(url) {
                let mut file = File::create(&config_file_path);
                file.expect("Error beim write all config.json")
                    .write_all(&response.bytes().expect("Error beim Lesen des response"))
                    .expect("Error beim schreiben der datei");

                println!("Datei erstellt von {}", url);
            } else {
                eprintln!("Cloud kann nicht starten");
                eprintln!("Das System kann die URL {} nicht abrufen", url);
                return None;
            }
        }

        // config.json deserialisieren
        let config_content = fs::read_to_string(&config_file_path).expect("Error beim lesen des config content");

        Some(serde_json::from_str(&config_content).expect("Error beim deserialisieren des config Inhalts"))
    }

    fn check_folder(exe_path: &PathBuf, config: &Value, cmd_prefix: &ColoredString){

        //task folder
        {
            let mut task_path = exe_path.clone();
            task_path.push(config["path"]["task"].as_str().expect("Error beim Lesen des path der config datei"));
            if !task_path.exists() {
                Bx::create_path(&task_path);
                println!("{} Task ordner erfolgreich erstellt {:?}", cmd_prefix, task_path);
            }
        }

        //template folder
        {
            let mut template_path = exe_path.clone();
            template_path.push(config["path"]["template"].as_str().expect("Error beim Lesen des path der config datei"));
            if !template_path.exists() {
                Bx::create_path(&template_path);
                println!("{} {:?} erfolgreich erstellt",cmd_prefix ,template_path);
            }
        }

        //service temp folder
        {
            let mut service_temp_path = exe_path.clone();
            service_temp_path.push(config["path"]["service"]["temp"].as_str().expect("Error beim Lesen des path der config datei"));
            if !service_temp_path.exists() {
                Bx::create_path(&service_temp_path);
                println!("{} {:?} erfolgreich erstellt",cmd_prefix ,service_temp_path);
            }
        }

        //service static folder
        {
            let mut service_static_path = exe_path.clone();
            service_static_path.push(config["path"]["service"]["static"].as_str().expect("Error beim Lesen des path der config datei"));
            if !service_static_path.exists() {
                Bx::create_path(&service_static_path);
                println!("{} {:?} erfolgreich erstellt",cmd_prefix ,service_static_path);
            }
        }

        //software files
        {
            let mut software_files_path = exe_path.clone();
            software_files_path.push(config["path"]["config"]["software_files"].as_str().expect("Error beim lesen des path der seoftware_files foldes"));
            if !software_files_path.exists() {
                Bx::create_path(&software_files_path);
                println!("{} {:?} erfolgreich erstellt", cmd_prefix, software_files_path);
            }
        }

    }

    fn check_link(exe_path: &PathBuf, config: &Value, cmd_prefix: &ColoredString) -> bool {
        // software.json link
        {
            let mut config_software_path = exe_path.clone();
            config_software_path.push(config["path"]["config"]["software"].as_str().expect("Error beim Lesen des path der config datei"));
            if !config_software_path.exists() {
                Bx::create_path(&config_software_path);
                println!("{} Config Ordner erfolgreich erstellt {:?}", cmd_prefix, config_software_path);
            }
            config_software_path.push("software.json");
            if !config_software_path.exists() {
                let url = "http://37.114.62.121/cloud/default_file/config/software.json";
                if let Ok(response) = get(url) {
                    let mut file = File::create(&config_software_path);
                    file.expect("Error beim Erstellen der Datei").write_all(&response.bytes().expect("Error beim Lesen des response")).expect("Error beim Schreiben der Datei");
                    println!("{} Datei erstellt von {}", cmd_prefix, url);
                } else {
                    eprintln!("Software file kann nicht heruntergeladen werden");
                    eprintln!("Bitte stellen sie sicher das sie zugriff auf {} haben", url);
                    return false;
                }
            }
        }
        // task.json link
        {
            let mut config_task_path = exe_path.clone();
            config_task_path.push(config["path"]["config"]["default_task"].as_str().expect("Error beim Lesen des path der config datei"));
            if !config_task_path.exists() {
                Bx::create_path(&config_task_path);
                println!("{} Config Ordner erfolgreich erstellt {:?}", cmd_prefix, &config_task_path);
            }
            config_task_path.push("task.json");
            if !config_task_path.exists() {
                let url = "http://37.114.62.121/cloud/default_file/config/task.json";
                if let Ok(response) = get(url) {
                    let mut file = File::create(&config_task_path);
                    file.expect("Error beim Erstellen der Datei").write_all(&response.bytes().expect("Error beim Lesen des response")).expect("Error beim Schreiben der Datei");
                    println!("{} Datei erstellt von {}", cmd_prefix, url);
                } else {
                    eprintln!("task default file kann nicht heruntergeladen werden");
                    eprintln!("Bitte stellen sie sicher das sie zugriff auf {} haben", url);
                    return false;
                }
            }
        }
        return true;
    }
}
fn extract_and_install_links(software_type: &str, software_links: &Map<String, Value>){
    let install_dir = Config::get_software_files_path();
    let cmd_prefix = Config::get_prefix();

    // Iteriere durch die Kategorien (self, server, proxy)
    // Iteriere durch die Software-Links in diesem Software-Typ (Kategorie)
    for (software_name, software_link_value) in software_links.iter() {
        if let Some(software_link) = software_link_value.as_str() {
            let software_dir = install_dir.join(software_type);
            install_software(software_link, software_name, software_type, &cmd_prefix);

        } else {
            println!(
                "{} Ungültiger Link für Software: {}",
                cmd_prefix, software_name
            );
        }
    }
}


fn install_software(
    software_link: &str,
    software_name: &str,
    software_type: &str,
    cmd_prefix: &ColoredString,
) {
    let install_dir = Config::get_software_files_path();
    if software_link.starts_with("local:") {
        // Die Software ist lokal verfügbar
        let local_path = &software_link[6..]; // Entferne das "local:" Präfix
        let local_file_path = install_dir.join(local_path);

        if local_file_path.exists() {
            println!(
                "{} Lokale Software '{}' ist bereits installiert: {:?}",
                cmd_prefix, software_name, local_file_path
            );
        } else {
            println!(
                "{} Installiere lokale Software '{}' von: {}",
                cmd_prefix, software_name, local_path
            );
            // Führe die Installation von der lokalen Quelle durch
            // Hier kannst du die Dateiendung aus local_path extrahieren, wenn benötigt
        }
    } else {
        // Die Software soll von einem externen Link installiert werden
        let file_extension = software_link
            .rsplitn(2, '.')
            .next()
            .unwrap_or("dat"); // Wenn keine Dateiendung gefunden wurde, verwenden Sie "dat"

        let file_name = format!("{}.{}", software_name, file_extension);
        let software_dir = install_dir.join(software_type);

        if !software_dir.exists() {
            fs::create_dir_all(&software_dir)
                .expect("Fehler beim Erstellen des Verzeichnisses");
        }

        let external_file_path = software_dir.join(&file_name);

        if external_file_path.exists() {
            println!(
                "{} Externe Software '{}' ist bereits installiert: {:?}",
                cmd_prefix, software_name, external_file_path
            );
        } else {
            println!(
                "{} Installiere Software '{}' von externem Link: {}",
                cmd_prefix, software_name, software_link
            );

            install_software_from_external_link(software_link, software_name, &external_file_path);
            // Führe die Installation von der externen Quelle durch und speichere sie in external_file_path
            // Du kannst z.B. die reqwest-Bibliothek verwenden, um den Download durchzuführen
        }
    }
}

fn install_software_from_external_link(software_link: &str, software_name: &str, target_path: &PathBuf) {
    let cmd_prefix = Config::get_prefix();
    let install_dir = Config::get_software_files_path();
    let file_extension = software_link
        .rsplitn(2, '.')
        .next()
        .unwrap_or("dat"); // Wenn keine Dateiendung gefunden wurde, verwenden Sie "dat"

    let file_name = format!("{}.{}", software_name, file_extension);
    let external_file_path = install_dir.join(&file_name);

    if external_file_path.exists() {
        println!(
            "{} Externe Software '{}' ist bereits installiert: {:?}",
            cmd_prefix, software_name, external_file_path
        );
    } else {
        println!(
            "{} Installiere Software '{}' von externem Link: {}",
            cmd_prefix, software_name, software_link
        );

        // Versuche, die Software von der externen URL herunterzuladen
        match get(software_link) {
            Ok(response) => {
                if response.status().is_success() {
                    // Erstelle die Datei und schreibe die heruntergeladenen Daten hinein
                    let mut file = File::create(&target_path)
                        .expect("Fehler beim Erstellen der Datei");

                    let bytes = response.bytes().expect("Fehler beim Lesen der Antwort");
                    file.write_all(&bytes)
                        .expect("Fehler beim Schreiben der Datei");

                    println!(
                        "{} Erfolgreich installierte Software '{}' unter: {:?}",
                        cmd_prefix, software_name, external_file_path
                    );
                } else {
                    println!(
                        "{} Fehler beim Herunterladen der Software '{}' von externem Link: Statuscode {}",
                        cmd_prefix,
                        software_name,
                        response.status()
                    );
                }
            }
            Err(err) => {
                println!(
                    "{} Fehler beim Herunterladen der Software '{}' von externem Link: {}",
                    cmd_prefix, software_name, err
                );
            }
        }
    }
}


/*
fn extract_links(json_obj: &Map<String, Value>) -> Vec<String> {
    let mut links = Vec::new();
    for (_, value) in json_obj {
        if let Some(url) = value.as_str() {
            links.push(url.to_string());
        }
    }
    links
}*/