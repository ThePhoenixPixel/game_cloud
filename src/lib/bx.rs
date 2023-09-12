use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
pub struct Bx;

impl Bx {
    pub fn create_path(path: &Path) {
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
}