use std::path::PathBuf;
use serde::Serialize;
use crate::config::Config;

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

    pub fn set_template(&mut self, template:String){
        self.template = template;
    }

    //name
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    //priority
    pub fn get_priority(&self) -> &u32 {
        &self.priority
    }

    pub fn set_priority(&mut self, priority: u32) {
        self.priority = priority;
    }

    pub fn get_path(&self) -> PathBuf{
        //PathBuf::from(format!("{}{}{}", Config::get_template_path().to_string_lossy().to_owned(),self.template, self.name))
        PathBuf::from(&Config::get_template_path())
            .join(&self.template)
            .join(&self.name)
    }
}