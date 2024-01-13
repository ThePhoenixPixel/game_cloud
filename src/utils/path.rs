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

}