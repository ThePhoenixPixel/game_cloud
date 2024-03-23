use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;

pub struct Bx;

impl Bx {
    pub fn create_path(path: &PathBuf) {
        let mut current_path = PathBuf::new();

        for component in path.components() {
            current_path.push(component);

            if !current_path.exists() {
                fs::create_dir(&current_path).expect("Fehler beim Erstellen des Ordners");
            }
        }
    }

    pub fn copy_folder_contents(from: &PathBuf, to: &PathBuf) -> Result<(), Box<dyn Error>> {
        // Für jede Datei/Verzeichnis im Quellordner
        for entry in fs::read_dir(from)? {
            let entry = entry?;
            let entry_path = entry.path();
            let target_path = to.join(entry_path.file_name().ok_or("Invalid file name")?);

            // Wenn es sich um ein Verzeichnis handelt, rufe die Funktion rekursiv auf
            if entry_path.is_dir() {
                fs::create_dir_all(&target_path)?;
                Bx::copy_folder_contents(&entry_path, &target_path)?;
            } else {
                fs::copy(&entry_path, &target_path)?;
            }
        }
        Ok(())
    }

    pub fn extract_extension_from_url(url: &String) -> Option<String> {
        if let Ok(url) = reqwest::Url::parse(url) {
            if let Some(file_name) = url.path_segments().and_then(|segments| segments.last()) {
                if let Some(extension) = Path::new(file_name).extension() {
                    return Some(extension.to_string_lossy().to_string());
                }
            }
        }
        None
    }

    pub fn extract_filename_from_url(url: &str) -> Option<String> {
        return Option::from(
            match url.rsplit('/').next() {
                Some(file_name) => file_name,
                None => return None,
            }
                .to_string(),
        );
    }

    pub fn extract_filename_from_pathbuf(path: &PathBuf) -> Option<String> {
        if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
            // Die letzten Teile der Pfadzeichenkette nach Pfadtrennern ('\') oder ('/') trennen und ausgeben
            let parts: Vec<&str> = file_name.split(|c| c == '/' || c == '\\').collect();
            if let Some(last_part) = parts.last() {
                return Some(last_part.to_string());
            }
        }
        None
    }

    pub fn get_last_folder_name(path: &PathBuf) -> String {
        if let Some(file_name) = path.file_name() {
            if let Some(name_str) = file_name.to_str() {
                return name_str.to_string();
            }
        }
        String::new()
    }

    pub fn download_file(
        url: &str,
        file_path: &PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        // url = http://domain.com/test.download
        // file_path = folder/file.test
        if file_path.exists() {
            return Ok(());
        }

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .no_brotli()
            .build()?;
        let response = client.get(url).send()?;
        // check ob response 2xx
        if !response.status().is_success() {
            return Err("Error response from Server".into());
        }

        let mut file = File::create(&file_path)?;

        file.write_all(&response.bytes()?)?;

        Ok(())
    }
}
