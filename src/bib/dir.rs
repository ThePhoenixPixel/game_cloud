use std::{fs::{self}, path::Path};

pub struct Dir;

impl Dir {
     
    pub fn create(dir_name: &str){
        let path = Path::new(dir_name);
        if path.exists() && path.is_dir() {
            println!("Der Ordner '{}' exsestiert schon", dir_name);
            
        }else{
            fs::create_dir_all(dir_name);
        }
    }

    pub fn enter(dir_name: &str){

        let path = Path::new(dir_name);

        if path.exists() && path.is_dir() {
            println!("Im Ordner '{}'",dir_name);

        }else if path.is_file() {
            print!("Dies ist eine datei");

        }else {
            print!("weder ein ordner noch eine datei");
        }
    }

    pub fn delete(dir_name: &str){
        let path = Path::new(dir_name);

        if path.exists() && path.is_dir() {
            fs::remove_dir_all(dir_name);
            println!("Ordner '{}' erfolgreich gelöscht", dir_name);

        }else if path.is_file() {
            print!("Dies ist eine datei bitte benutzen die dafür die entsprechende methode");

        }else {
            print!("weder ein ordner noch eine datei");
        }
        
    }
}