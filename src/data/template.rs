use crate::config::Config;
use crate::data::task::Task;
use crate::lib::bx::Bx;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Template {
    pub template: String,
    pub name: String,
    pub priority: u32,
}

impl Template {
    pub fn new() -> Template {
        Template {
            template: "templatename".to_string(),
            name: "default".to_string(),
            priority: 1,
        }
    }

    //template
    pub fn get_template(&self) -> &String {
        &self.template
    }

    pub fn set_template(&mut self, template: &String) {
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

    pub fn create_by_task(task: &Task) {
        let mut template_path = Config::get_template_path();
        template_path.push(task.get_name());
        template_path.push("default");

        if !template_path.exists() {
            Bx::create_path(&template_path);
        }
    }

    pub fn create_by_self(&self) {
        Bx::create_path(&self.get_path());
    }
}
