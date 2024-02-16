use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use crate::log_error;

use crate::utils::logger::Logger;
use crate::sys_config::cloud_config::CloudConfig;
use crate::sys_config::software_config::{SoftwareConfig, SoftwareName};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Software {
    pub software_type: String,
    pub name: String,
}

impl Software {
    pub fn new(software_type: &String, name: &String) -> Software {
        Software {
            software_type: software_type.clone(),
            name: name.clone(),
        }
    }

    pub fn get_software_from_software_config(&self) -> Option<SoftwareName> {
        let software_type = match SoftwareConfig::get().get_software_type(self.get_software_type())
        {
            Some(software_type) => software_type,
            None => return None,
        };
        return match software_type.get_software_name(&self.get_name()) {
            Some(software) => Some(software),
            None => None,
        };
    }

    //software type
    pub fn get_software_type(&self) -> &String {
        &self.software_type
    }

    pub fn set_software_type(&mut self, software_type: &String) {
        self.software_type = software_type.clone();
    }

    //name
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_name_with_ext(&self) -> String {
        let name = self.get_name();
        let binding = self.get_software_url();
        let binding = match binding {
            Some(binding) => binding,
            None => {
                log_error!("Error in Software.rs can not get software url");
                String::new()
            }
        };
        let link = Path::new(&binding);
        return if let Some(ext) = link.extension().and_then(|ext| ext.to_str()) {
            format!("{}.{}", name, ext)
        } else {
            // Fallback, wenn keine Dateiendung gefunden wurde
            format!("{}", name)
        };
    }

    pub fn set_name(&mut self, name: &String) {
        self.name = name.clone();
    }

    pub fn get_software_url(&self) -> Option<String> {
        let software_type = match SoftwareConfig::get().get_software_type(self.get_software_type())
        {
            Some(software_type) => software_type,
            None => return None,
        };
        return match software_type.get_software_name(&self.get_name()) {
            Some(software) => Some(software.get_download()),
            None => None,
        };
    }

    pub fn get_software_file_path(&self) -> PathBuf {
        let mut software_path = CloudConfig::get()
            .get_cloud_path()
            .get_system_folder()
            .get_software_files_folder_path();
        software_path.push(&self.get_software_type());
        software_path.push(self.get_name_with_ext());
        software_path
    }
}
