use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::{fs, io};

use crate::core::installer::Installer;
use crate::core::service::Service;
use crate::core::software::Software;
use crate::core::template::Template;
use crate::lib::bx::Bx;
use crate::sys_config::cloud_config::CloudConfig;
use crate::utils::logger::Logger;
use crate::utils::path::Path;
use crate::{log_error, log_info};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    // Task Struktur
    name: String,
    split: char,
    delete_on_stop: bool,
    static_service: bool,
    nodes: Vec<String>,
    software: Software,
    max_ram: u32,
    start_port: u32,
    min_service_count: u32,
    groups: Vec<String>,
    installer: Installer,
    templates: Vec<Template>,
}

impl Task {
    pub fn new() -> Task {
        let default_task_path = CloudConfig::get()
            .get_cloud_path()
            .get_system_folder()
            .get_default_task_path();

        let default_task_content =
            fs::read_to_string(&default_task_path).unwrap_or_else(|_| "".to_string());
        let default_task_config: serde_json::Value =
            serde_json::from_str(&default_task_content).unwrap_or_else(|_| serde_json::Value::Null);

        let name = default_task_config["name"]
            .as_str()
            .unwrap_or("taskname")
            .to_string();
        let mut split = '-';
        let split_temp: String = default_task_config["split"]
            .as_str()
            .unwrap_or("-")
            .to_string();
        if split_temp.len() == 1 {
            // Überprüfen, ob der String nur aus einem Zeichen besteht
            split = split_temp.chars().next().unwrap();
        } else {
            println!("Fehler: Der 'split'-Wert ist kein einzelnes Zeichen.");
        }

        let delete_on_stop = default_task_config["delete_on_stop"]
            .as_bool()
            .unwrap_or(true);
        let static_service = default_task_config["static_service"]
            .as_bool()
            .unwrap_or(false);
        let max_ram = default_task_config["max_ram"].as_u64().unwrap_or(2048) as u32;
        let start_port = default_task_config["start_port"].as_u64().unwrap_or(40000) as u32;
        let min_service_count = default_task_config["min_service_count"]
            .as_u64()
            .unwrap_or(0) as u32;
        // let installer = default_task_config["installer"].to_string();

        let installer = Installer::InstallAll;
        let groups = Vec::new();
        let software = Software::new(&"server".to_string(), &"paper".to_string());
        let templates = vec![Template::new()];

        let task = Task {
            name,
            split,
            delete_on_stop,
            static_service,
            nodes: Vec::new(),
            software,
            max_ram,
            start_port,
            min_service_count,
            groups,
            installer,
            templates,
        };

        task
    }

    // Getter and Setter for name
    pub fn get_name(&self) -> String {
        self.name.parse().unwrap()
    }

    pub fn change_name(&mut self, name: String) {
        self.name = name;
        self.save_to_file();
    }

    //getter und setter for split
    pub fn get_split(&self) -> char {
        self.split
    }

    pub fn set_split(&mut self, split: &char) {
        self.split = split.clone();
        self.save_to_file()
    }

    // Getter and Setter for delete_on_stop
    pub fn get_delete_on_stop(&self) -> bool {
        self.delete_on_stop
    }

    pub fn set_delete_on_stop(&mut self, delete_on_stop: bool) {
        self.delete_on_stop = delete_on_stop;
        self.save_to_file();
    }

    // Getter and Setter for static_service
    pub fn get_static_service(&self) -> bool {
        self.static_service
    }

    pub fn set_static_service(&mut self, static_service: bool) {
        self.static_service = static_service;
        self.save_to_file();
    }

    // Getter and Setter for nodes
    pub fn get_nodes(&self) -> Vec<String> {
        self.nodes.clone()
    }

    pub fn add_node(&mut self, node: String) {
        self.nodes.push(node);
        self.save_to_file();
    }

    pub fn remove_node(&mut self, node: &String) {
        if let Some(index) = self.nodes.iter().position(|n| n == node) {
            self.nodes.remove(index);
        }
        self.save_to_file();
    }

    pub fn clear_nodes(&mut self) {
        self.nodes.clear();
        self.save_to_file();
    }
    // Getter and Setter for software
    pub fn get_software(&self) -> Software {
        self.software.clone()
    }

    pub fn set_software(&mut self, software: Software) {
        self.software = software;
        self.save_to_file();
    }

    //max ram
    pub fn get_max_ram(&self) -> u32 {
        self.max_ram
    }

    pub fn set_max_ram(&mut self, max_ram: &u32) {
        self.max_ram = max_ram.clone();
        self.save_to_file();
    }

    // Getter and Setter for start_port
    pub fn get_start_port(&self) -> u32 {
        self.start_port
    }

    pub fn set_start_port(&mut self, start_port: u32) {
        self.start_port = start_port;
        self.save_to_file();
    }

    // Getter and Setter for min_service_count
    pub fn get_min_service_count(&self) -> u32 {
        self.min_service_count
    }

    pub fn set_min_service_count(&mut self, min_service_count: u32) {
        self.min_service_count = min_service_count;
        self.save_to_file();
    }

    // Getter and Setter for groups
    pub fn get_groups(&self) -> &Vec<String> {
        &self.groups
    }

    pub fn add_group(&mut self, group: &String) {
        self.groups.push(group.clone());
        self.save_to_file();
    }

    pub fn remove_group(&mut self, group: &String) {
        if let Some(index) = self.groups.iter().position(|g| g == group) {
            self.groups.remove(index);
        }
        self.save_to_file();
    }

    pub fn clear_groups(&mut self) {
        self.groups.clear();
        self.save_to_file();
    }

    // Getter für installer
    pub fn get_installer(&self) -> &Installer {
        &self.installer
    }

    // Setter für installer
    pub fn set_installer(&mut self, installer: &Installer) {
        self.installer = installer.clone();
        self.save_to_file();
    }

    pub fn get_templates(&self) -> Vec<Template> {
        self.templates.clone()
    }

    pub fn add_template(&mut self, template: Template) {
        self.templates.push(template);
        self.save_to_file();
    }

    pub fn remove_template(&mut self, template: Template) {
        if let Some(index) = self.templates.iter().position(|t| t == &template) {
            self.templates.remove(index);
        }
        self.save_to_file();
    }

    pub fn clear_templates(&mut self) {
        self.templates.clear();
        self.save_to_file();
    }

    pub fn is_exist(name: String) -> bool {
        if Task::get_task(name).is_some() {
            true
        } else {
            false
        }
    }

    pub fn to_json(&self) -> Option<serde_json::Value> {
        let json_string = match serde_json::to_string_pretty(self) {
            Ok(json_string) => json_string,
            Err(e) => {
                log_error!("{}", e.to_string());
                return None;
            }
        };

        return match serde_json::from_str(json_string.as_str()) {
            Ok(json) => Some(json),
            Err(e) => {
                log_error!("{}", e.to_string());
                None
            }
        };
    }

    // get task object from name
    pub fn get_task(name: String) -> Option<Task> {
        let task_path = CloudConfig::get().get_cloud_path().get_task_folder_path();

        let files_name = Path::get_files_name_from_path(&task_path);

        // iter list of files Name
        for file_name in files_name {
            let task = match Task::from_path(&task_path.join(&file_name)) {
                Ok(task) => task,
                Err(e) => {
                    log_error!("{}", e.to_string());
                    return None;
                }
            };

            // check name of the task is the same of the param name
            if task.get_name() == name {
                return Some(task);
            }
        }
        None
    }

    // from path to task object
    pub fn from_path(path: &PathBuf) -> io::Result<Task> {
        let mut file = File::open(path)?;
        let mut content = String::new();

        file.read_to_string(&mut content)?;

        let task: Task = serde_json::from_str(&content)?;

        Ok(task)
    }

    pub fn get_task_all() -> Vec<Task> {
        let task_path = CloudConfig::get().get_cloud_path().get_task_folder_path();
        let mut tasks: Vec<Task> = Vec::new();

        if task_path.exists() && task_path.is_dir() {
            if let Ok(entries) = fs::read_dir(task_path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Some(file_name) = entry.file_name().to_str() {
                            if let Some(name) = file_name.strip_suffix(".json") {
                                tasks.push(match Task::get_task(name.to_string()) {
                                    Some(task) => task,
                                    None => break,
                                });
                            }
                        }
                    }
                }
            }
        }

        tasks
    }

    pub fn setup(
        &mut self,
        name: String,
        delete_on_stop: bool,
        static_service: bool,
        nodes: Vec<String>,
        software: Software,
        max_ram: u32,
        start_port: u32,
        min_service_count: u32,
        groups: Vec<String>,
        templates: Vec<Template>,
    ) {
        self.name = name;
        self.delete_on_stop = delete_on_stop;
        self.static_service = static_service;
        self.nodes = nodes;
        self.software = software;
        self.max_ram = max_ram;
        self.start_port = start_port;
        self.min_service_count = min_service_count;
        self.groups = groups;
        self.templates = templates;
        self.save_to_file();
    }

    pub fn save_to_file(&self) {
        let serialized_task =
            serde_json::to_string_pretty(&self).expect("Error beim Serialisieren der Task");
        let task_path = CloudConfig::get()
            .get_cloud_path()
            .get_task_folder_path()
            .join(format!("{}.json", self.get_name()));

        if !task_path.exists() {
            Template::create_by_task(&self);
        }

        let mut file = File::create(&task_path).expect("Error beim Erstellen der Task-Datei");
        file.write_all(serialized_task.as_bytes())
            .expect("Error beim Schreiben in die Task-Datei");
    }

    pub fn delete_as_file(&self) {
        let mut task_path = CloudConfig::get().get_cloud_path().get_task_folder_path();
        task_path.push(format!("{}.json", &self.name));

        fs::remove_file(task_path).expect("Error bei  removen der task datei");
    }

    pub fn reload() {
        Service::reload();
    }

    pub fn prepared_to_services(&self) {
        let templates = &self.templates;
        let select_template = select_template_with_priority(&templates);

        //check ob es template gibt
        if select_template.is_some() {
        } else {
            println!(
                "{} Kein Template gefunden für Task: {}",
                "GameCloud in task.rs fn prepare_to_service",
                &self.get_name()
            );
            return;
        }
        //make option template to template
        let template = select_template.unwrap();

        //hier temp oder static
        {
            //temp service
            let mut target_folder_name = format!("{}-1", &template.template);
            let mut target_path = CloudConfig::get()
                .get_cloud_path()
                .get_service_folder()
                .get_temp_folder_path()
                .join(&target_folder_name);

            // Überprüfen, ob der Zielordner bereits existiert, und erhöhen Sie die Nummer, falls erforderlich
            let mut folder_number = 1;
            while target_path.exists() {
                folder_number += 1;
                target_folder_name = format!("{}-{}", &template.template, folder_number);
                target_path = CloudConfig::get()
                    .get_cloud_path()
                    .get_service_folder()
                    .get_temp_folder_path()
                    .join(&target_folder_name);
            }

            // Hier wird der Zielordner erstellt, wenn er nicht existiert
            fs::create_dir_all(&target_path).expect("Fehler beim Erstellen des Zielordners");

            //println!("{:?}", &template.get_path());
            //println!("{:?}", &target_path);

            // Jetzt kannst du den Inhalt aus dem Template-Pfad in den Zielordner kopieren
            Bx::copy_folder_contents(&template.get_path(), &target_path)
                .expect("Fehler beim Kopieren des Templates");

            //println!("{:?}", &self.get_software().get_software_file_path());

            let mut target_server_file_path = target_path.clone();
            target_server_file_path.push(&self.get_software().get_name_with_ext());
            fs::copy(
                &self.get_software().get_software_file_path(),
                &target_server_file_path,
            )
            .expect("Erro beim copy der server datei");

            println!(
                "{} Template wurde in Zielordner kopiert: {:?}",
                "GameCloud in task.rs fn prepare_to_service", &target_path
            );
        }
    }
    //get temp or static for the service
    pub fn get_service_path(&self) -> PathBuf {
        let path = if self.static_service {
            CloudConfig::get()
                .get_cloud_path()
                .get_service_folder()
                .get_static_folder_path()
        } else {
            CloudConfig::get()
                .get_cloud_path()
                .get_service_folder()
                .get_temp_folder_path()
        };
        path
    }

    //print the task object in cmd
    pub fn print(&self) {
        log_info!("--------> Task Info <--------");
        log_info!("name: {}", self.get_name());
        log_info!("split: {}", self.get_split());
        log_info!("delete_on_stop: {}", self.get_delete_on_stop());
        log_info!("static_service: {}", self.get_static_service());
        log_info!("nodes: {:?}", self.get_nodes());
        log_info!("software: ");
        log_info!(
            "     software_type: {}",
            self.get_software().get_software_type()
        );
        log_info!("     name: {}", self.get_software().get_name());
        log_info!("max_ram: {}", self.get_max_ram());
        log_info!("start_port: {}", self.get_start_port());
        log_info!("min_service_count: {}", self.get_min_service_count());
        log_info!("groups: {:?}", self.get_groups());
        log_info!("installer: {:?}", self.get_installer());
        log_info!("templates: ");
        for template in self.get_templates() {
            log_info!("     template: {}", template.get_template());
            log_info!("     name: {}", template.get_name());
            log_info!("     priority: {}", template.get_priority());
        }
        log_info!("-----------------------------");
    }
}

fn select_template_with_priority(templates: &[Template]) -> Option<&Template> {
    let mut rng = rand::thread_rng();
    let total_priority: u32 = templates.iter().map(|t| t.priority).sum();
    let mut rand_value = rng.gen_range(1..=total_priority);

    for template in templates {
        if rand_value <= template.priority {
            return Some(template);
        }
        rand_value -= template.priority;
    }
    None
}
