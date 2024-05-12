use std::fs;
use std::path::PathBuf;

pub struct Path;

impl Path {
    //
    pub fn get_files_name_from_path(path: &PathBuf) -> Vec<String> {
        let mut files_name: Vec<String> = Vec::new();
        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        files_name.push(file_name.to_string());
                    }
                }
            }
        }
        files_name
    }
    pub fn get_folders_name_from_path(path: &PathBuf) -> Vec<String> {
        let mut folders_name: Vec<String> = Vec::new();
        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(folder_name) = entry.file_name().to_str() {
                        if let Some(metadata) = entry.metadata().ok() {
                            if metadata.is_dir() {
                                folders_name.push(folder_name.to_string());
                            }
                        }
                    }
                }
            }
        }
        folders_name
    }
}
