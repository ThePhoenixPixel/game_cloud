use std::fs;
use std::io::Write;
use rand::Rng;
use serde::Serialize;
use crate::config::Config;
use crate::data::software::Software;
use crate::data::template::Template;
use crate::lib::bx::Bx;

#[derive(Serialize)]
pub struct Task {
    // Task Struktur
    name: String,
    delete_on_stop: bool,
    static_service: bool,
    nodes: Vec<String>,
    software: Software,
    start_port: u32,
    min_service_count: u32,
    groups: Vec<String>,
    templates: Vec<Template>,
}

impl Task{
    pub fn new() -> Task {
        let default_task_path = Config::get_config_default_task_path();

        let default_task_content = fs::read_to_string(&default_task_path).unwrap_or_else(|_| "".to_string());
        let default_task_config: serde_json::Value = serde_json::from_str(&default_task_content).unwrap_or_else(|_| serde_json::Value::Null);

        let name = default_task_config["name"].as_str().unwrap_or("taskname").to_string();
        let delete_on_stop = default_task_config["delete_on_stop"].as_bool().unwrap_or(true);
        let static_service = default_task_config["static_service"].as_bool().unwrap_or(false);
        let start_port = default_task_config["start_port"].as_u64().unwrap_or(40000) as u32;
        let min_service_count = default_task_config["min_service_count"].as_u64().unwrap_or(0) as u32;

        let mut groups = Vec::new();
        let software = Software::new();
        let templates = vec![Template::new()];

        Task {
            name,
            delete_on_stop,
            static_service,
            nodes: Vec::new(),
            software,
            start_port,
            min_service_count,
            groups,
            templates,
        }
    }

    // Getter and Setter for name
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn change_name(&mut self, name: String) {
        self.name = name;
        //self.save_to_file();
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
    pub fn get_nodes(&self) -> &Vec<String> {
        &self.nodes
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

    // Getter and Setter for software
    pub fn get_software(&self) -> &Software {
        &self.software
    }

    pub fn set_software(&mut self, software: Software) {
        self.software = software;
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

    pub fn add_group(&mut self, group: String) {
        self.groups.push(group);
        self.save_to_file();
    }



    // Templatte/s
    /*pub fn get_template(&self) -> String{

    }
    pub fn get_templates(&self) -> &Vec<Template> {
        &self.templates
    }

    pub fn set_templates(&mut self, templates: Template) {
        self.templates = templates;
    }*/

    pub fn is_exist(name: String) -> bool {
        if let Some(task) = Task::get_task(name) {
            true
        } else {
            false
        }
    }

    pub fn get_task(name: String) -> Option<Task> {
        let task_path = Config::get_task_path();

        // YAML-Dateien lesen
        let json_files = fs::read_dir(&task_path)
            .expect("Fehler beim Lesen des Task-Ordners")
            .filter_map(|entry| {
                let entry = entry.expect("Fehler beim Lesen des Verzeichniseintrags");
                let file_path = entry.path();
                if file_path.is_file() {
                    Some(file_path)
                } else {
                    None
                }
            });

        // Nach dem Task mit dem angegebenen Namen suchen
        for file_path in json_files {
            // Dateiinhalt lesen
            let file_content = fs::read_to_string(&file_path)
                .expect("Fehler beim Lesen der YAML-Datei");

            let config: serde_json::Value = serde_json::from_str(&file_content)
                .expect("Fehler beim Deserialisieren der JSON-Datei");

            let task_name = config["name"].as_str();

            if let Some(task_name_str) = task_name {
                if task_name_str == name {
                    let mut task = Task::new();

                    // Setup-Methode aufrufen und die korrekten Parameter übergeben
                    task.setup(
                        name.to_string(),
                        config["delete_on_stop"].as_bool().unwrap_or(true),
                        config["static_service"].as_bool().unwrap_or(false),
                        Vec::new(), // Hier können die Nodes aus der Config hinzugefügt werden
                        Software {
                            software_type: config["software"]["software_type"].as_str().unwrap_or("server").to_string(),
                            name: config["software"]["name"].as_str().unwrap_or("paper").to_string(),
                            max_ram: config["software"]["max_ram"].as_u64().unwrap_or(1024) as u32,
                        },
                        config["start_port"].as_u64().unwrap_or(40000) as u32,
                        config["min_service_count"].as_u64().unwrap_or(0) as u32,
                        Vec::new(), // Hier können die Groups aus der Config hinzugefügt werden
                        config["templates"]
                            .as_array()
                            .unwrap_or(&vec![]) // Hier werden die Templates aus der Config als Vektor von JSON-Objekten behandelt
                            .iter()
                            .map(|template| Template {
                                // Hier wird jedes JSON-Objekt in ein Template-Objekt umgewandelt
                                template: template["template"].as_str().unwrap_or("taskname").to_string(),
                                name: template["name"].as_str().unwrap_or("default").to_string(),
                                priority: template["priority"].as_u64().unwrap_or(1) as u32,
                            })
                            .collect::<Vec<Template>>(),
                    );

                    return Some(task);
                }
            }

        }
        None // Wenn kein passender Task gefunden wurde
    }

    pub fn get_task_all() -> Vec<String> {
        let task_path = Config::get_task_path();

        let mut task_names = Vec::new();

        if task_path.exists() && task_path.is_dir() {
            if let Ok(entries) = fs::read_dir(task_path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Some(file_name) = entry.file_name().to_str() {
                            if let Some(name) = file_name.strip_suffix(".json") {
                                task_names.push(name.to_string());
                            }
                        }
                    }
                }
            }
        }

        task_names
    }

    pub fn setup(&mut self, name: String,
                 delete_on_stop: bool,
                 static_service: bool,
                 nodes: Vec<String>,
                 software: Software,
                 start_port: u32,
                 min_service_count: u32,
                 groups: Vec<String>,
                 templates: Vec<Template>,)
    {
        self.name = name;
        self.delete_on_stop = delete_on_stop;
        self.static_service = static_service;
        self.nodes = nodes;
        self.software = software;
        self.start_port = start_port;
        self.min_service_count = min_service_count;
        self.groups = groups;
        self.templates = templates;
        self.save_to_file();
    }

    pub fn save_to_file(&self) {
        let serialized_task = serde_json::to_string_pretty(&self).expect("Error beim Serialisieren der Task");
        let task_path = Config::get_task_path().join(format!("{}.json", self.name));

        let mut file = fs::File::create(&task_path).expect("Error beim Erstellen der Task-Datei");
        file.write_all(serialized_task.as_bytes()).expect("Error beim Schreiben in die Task-Datei");
    }

    pub fn delete_as_file(&self){
        let mut task_path = Config::get_task_path();
        task_path.push(format!("{}.json", &self.name));

        fs::remove_file(task_path).expect("Error bei  removen der task datei");
    }

    pub fn reload(){

        let task_all = Task::get_task_all();

        println!("{:?}", task_all);

        for task_name in task_all {
            if let Some(task) = Task::get_task(task_name) {
                println!("{}", &task.get_name());
                if task.get_min_service_count() > 0 {
                    for _ in 0..task.get_min_service_count() {
                        println!("Dienst starten {}", &task.get_name());
                        //task.prepared_to_services();
                    }
                }
            } else {
                println!("{} task error", Config::get_prefix());
            }
        }

    }

    pub fn prepared_to_services(&self) {
        let templates = &self.templates;
        let mut select_template= select_template_with_priority(&templates);

        //check ob es template gibt
        if let Some(template) = select_template {

        } else {
            println!("{} Kein Template gefunden für Task: {}", Config::get_prefix(), &self.get_name());
            return;
        }
        //make option template to template
        let template = select_template.unwrap();

        //hier temp oder static
        {   //temp service
            let mut target_folder_name = format!("{}-1", &template.template);
            let mut target_path = Config::get_service_temp_path().join(&target_folder_name);

            // Überprüfen, ob der Zielordner bereits existiert, und erhöhen Sie die Nummer, falls erforderlich
            let mut folder_number = 1;
            while target_path.exists() {
                folder_number += 1;
                target_folder_name = format!("{}-{}", &template.template, folder_number);
                target_path = Config::get_service_temp_path().join(&target_folder_name);
            }

            // Hier wird der Zielordner erstellt, wenn er nicht existiert
            fs::create_dir_all(&target_path).expect("Fehler beim Erstellen des Zielordners");

            // Jetzt kannst du den Inhalt aus dem Template-Pfad in den Zielordner kopieren
            Bx::copy_folder_contents(&template.get_path(), &target_path).expect("Fehler beim Kopieren des Templates");

            println!("{} Template wurde in Zielordner kopiert: {:?}", Config::get_prefix(), &target_path);
        }

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