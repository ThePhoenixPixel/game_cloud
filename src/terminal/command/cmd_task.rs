use std::net::ToSocketAddrs;
use crate::data::software::Software;
use crate::data::task::Task;
use crate::logger::Logger;
use crate::sys_config::cloud_config::CloudConfig;
use crate::sys_config::software_config::SoftwareConfig;
use crate::terminal::command_manager::CommandManager;

pub struct CmdTask;

impl CommandManager for CmdTask {
    fn execute(args: Vec<&str>) {
        let arg1 = match args.get(1) {
            Some(arg) => arg.clone(),
            None => {
                Logger::warning("Bitte gebe ein ein der volgebedej  argumente an");
                return;
            }
        };
        print!("{}", arg1);
        match arg1 {
            "add" => add(args),
            "remove" => remove(args),
            "list" => list(),
            "info" => {}
            _ => {
                Logger::warning(
                    "Dies ist kein Gültiges argument verwende eins davon / add / remove / list",
                );
                return;
            }
        }
    }
    fn tab_complete(args: Vec<&str>) -> Vec<String> {
        todo!()
    }
}

fn info(args: Vec<&str>) {
    // command task info <name>
    let task_name = match args.get(1) {
        Some(task_name) => task_name.to_string(),
        None => {
            Logger::warning("Bitte gebe ein passenden task namen an");
            return;
        }
    };

    let task = match Task::get_task(task_name) {
        Some(task) => task,
        None => {
            Logger::warning("Bitte gebe ein task namen an der exsistiert");
        }
    };



}

fn add(args: Vec<&str>) {
    // command: task add <name> <software_type> <software_name>
    let task_name = match args.get(2) {
        Some(task_name) => task_name.to_string(),
        None => {
            Logger::warning("Bitte gib ein task namen an");
            return;
        }
    };

    if Task::is_exist(task_name.clone()) {
        Logger::warning("Diese task esistiert bereits");
        return;
    }

    let software_type = match args.get(3) {
        Some(software_type) => software_type.to_string(),
        None => {
            Logger::warning("Bitte gebe ein Software Type an");
            return;
        }
    };

    let software = match SoftwareConfig::get().get_software_type(&software_type.to_string()) {
        Some(software) => software,
        None => {
            Logger::warning("Bitte gebe ein SOftware Type an der exsistiert");
            return;
        }
    };

    let software_name = match args.get(4) {
        Some(software_name) => software_name.to_string(),
        None => {
            Logger::warning("BItte gebe ein software name an");
            return;
        }
    };

    match software.get_software_name(&software_name) {
        Some(_) => {}
        None => {
            Logger::warning("bitte gebe ein software namen an der exsistiert");
            return;
        }
    }

    let mut task = Task::new();
    task.change_name(task_name.to_string());
    task.set_software(Software::new(&software_type, &software_name));
    task.save_to_file();

    Logger::info("Task Erfolgreich erstellt");
}

fn remove(args: Vec<&str>) {
    // command task remove <name>

    let task_name = match args.get(1) {
        Some(task_name) => task_name.to_string(),
        None => {
            Logger::warning("bitte gebe ein task namen an den du löschen möchtest");
            return;
        }
    };

    let task = match Task::get_task(task_name) {
        Some(task) => task,
        None => {
            Logger::warning("Task nicht gefunden ");
            return;
        }
    };
    // delete the task
    task.delete_as_file();
    Logger::info("Task erfolgreich gelöscht");
}

fn list() {
    Logger::info("--------> Tasks <--------");
    for task in Task::get_task_all() {
        Logger::info(task.get_name().as_str());
    }
}
