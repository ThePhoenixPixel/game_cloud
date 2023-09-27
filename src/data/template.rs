use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use reqwest::blocking::get;
use serde::Serialize;
use crate::config::Config;
use crate::data::task::Task;
use crate::lib::bx::Bx;

#[derive(Serialize, Debug)]
pub struct Template {
    pub template: String,
    pub name: String,
    pub priority: u32,
}

impl Template {
    pub fn new() -> Template{
        Template{
            template: "templatename".to_string(),
            name: "default".to_string(),
            priority: 1,
        }
    }

    //template
    pub fn get_template(&self) -> &String {
        &self.template
    }

    pub fn set_template(&mut self, template: &String){
        self.template = template.clone();
    }

    //name
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: &String) {
        self.name = name.clone();
    }

    //priority
    pub fn get_priority(&self) -> &u32 {
        &self.priority
    }

    pub fn set_priority(&mut self, priority: &u32) {
        self.priority = priority.clone();
    }

    pub fn get_path(&self) -> PathBuf {
        PathBuf::from(&Config::get_template_path())
            .join(&self.template)
            .join(&self.name)
    }

    pub fn create_by_task(task: &Task){
        let mut template_path = Config::get_template_path();
        template_path.push(task.get_name());
        template_path.push("default");

        if !template_path.exists() {
            Bx::create_path(&template_path);
        }

        if let Some(server_file_url) = task.get_software().get_software_url() {

            if let Ok(response) = get(&server_file_url) {

                let mut server_file_path = template_path.clone();

                if let Some(server_file_endung) = extract_extension_from_url(&server_file_url){
                    server_file_path.push(format!("{}.{}", task.get_software().get_name(), server_file_endung));

                } else {
                    eprintln!("{} Ungültige URL", Config::get_prefix());
                }


                let mut file = File::create(&server_file_path);
                file.expect("Error beim write all")
                    .write_all(&response.bytes()
                        .expect("Error beim Lesen des response"))
                    .expect("Error beim schreiben der datei");

            } else {
                eprintln!("{} Das System kann die URL {} nicht abrufen", Config::get_prefix() ,  server_file_url);
                return;
            }

        } else {
            println!("{} Die software in der der Task ist ungültig", Config::get_prefix());
        }
    }

    pub fn create_by_self(&self) {

        Bx::create_path(&self.get_path());

    }
}

fn extract_filename_from_url(url: &String) -> Option<String> {
    if let Ok(url) = reqwest::Url::parse(url) {
        if let Some(file_name) = url.path_segments().and_then(|segments| segments.last()) {
            return Some(file_name.to_string());
        }
    }
    None
}
fn extract_extension_from_url(url: &String) -> Option<String> {
    if let Ok(url) = reqwest::Url::parse(url) {
        if let Some(file_name) = url.path_segments().and_then(|segments| segments.last()) {
            if let Some(extension) = std::path::Path::new(file_name).extension() {
                return Some(extension.to_string_lossy().to_string());
            }
        }
    }
    None
}


