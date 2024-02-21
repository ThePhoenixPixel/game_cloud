use crate::core::software::Software;
use crate::core::task::Task;
use crate::sys_config::software_config::SoftwareConfig;
use crate::terminal::command_manager::CommandManager;
use crate::utils::logger::Logger;
use crate::{log_info, log_warning};

pub struct CmdTask;

impl CommandManager for CmdTask {
    fn execute(args: Vec<&str>) {
        // get the first argument command task <arg1>
        let arg1 = match args.get(1) {
            Some(arg) => arg.clone(),
            None => {
                log_warning!("Bitte gebe ein ein der volgebedej  argumente an");
                return;
            }
        };

        match arg1 {
            "add" => add(args),
            "remove" => remove(args),
            "list" => list(),
            "info" => info(args),
            _ => {
                log_warning!(
                    "Dies ist kein Gültiges argument verwende eins davon / add / remove / list",
                );
                return;
            }
        }
    }
    fn tab_complete(_args: Vec<&str>) -> Vec<String> {
        todo!()
    }
}

fn info(args: Vec<&str>) {
    // command task info <name>
    let task_name = match args.get(2) {
        Some(task_name) => task_name.to_string(),
        None => {
            log_warning!("Bitte gebe ein passenden task namen an");
            return;
        }
    };

    let task = match Task::get_task(task_name) {
        Some(task) => task,
        None => {
            log_warning!("Bitte gebe ein task namen an der exsistiert");
            return;
        }
    };

    log_info!("--------> Task Info <--------");
    log_info!("name: {}", task.get_name());
    log_info!("split: {}", task.get_split());
    log_info!("delete_on_stop: {}", task.get_delete_on_stop());
    log_info!("static_service: {}", task.get_static_service());
    log_info!("nodes: {:?}", task.get_nodes());
    log_info!("software: ");
    log_info!(
        "     software_type: {}",
        task.get_software().get_software_type()
    );
    log_info!("     name: {}", task.get_software().get_name());
    log_info!("max_ram: {}", task.get_max_ram());
    log_info!("start_port: {}", task.get_start_port());
    log_info!("min_service_count: {}", task.get_min_service_count());
    log_info!("groups: {:?}", task.get_groups());
    log_info!("installer: {:?}", task.get_installer());
    log_info!("templates: ");
    for template in task.get_templates() {
        log_info!("     template: {}", template.get_template());
        log_info!("     name: {}", template.get_name());
        log_info!("     priority: {}", template.get_priority());
    }
}

fn add(args: Vec<&str>) {
    // command: task add <name> <software_type> <software_name>
    let task_name = match args.get(2) {
        Some(task_name) => task_name.to_string(),
        None => {
            log_warning!("Bitte gib ein task namen an");
            return;
        }
    };

    if Task::is_exist(task_name.clone()) {
        log_warning!("Diese task esistiert bereits");
        return;
    }

    let software_type = match args.get(3) {
        Some(software_type) => software_type.to_string(),
        None => {
            log_warning!("Bitte gebe ein Software Type an");
            return;
        }
    };

    let software = match SoftwareConfig::get().get_software_type(&software_type.to_string()) {
        Some(software) => software,
        None => {
            log_warning!("Bitte gebe ein SOftware Type an der exsistiert");
            return;
        }
    };

    let software_name = match args.get(4) {
        Some(software_name) => software_name.to_string(),
        None => {
            log_warning!("BItte gebe ein software name an");
            return;
        }
    };

    match software.get_software_name(&software_name) {
        Some(_) => {}
        None => {
            log_warning!("bitte gebe ein software namen an der exsistiert");
            return;
        }
    }

    let mut task = Task::new();
    task.change_name(task_name.to_string());
    task.set_software(Software::new(&software_type, &software_name));
    task.save_to_file();

    log_info!("Task Erfolgreich erstellt");
}

fn remove(args: Vec<&str>) {
    // command task remove <name>

    let task_name = match args.get(1) {
        Some(task_name) => task_name.to_string(),
        None => {
            log_warning!("bitte gebe ein task namen an den du löschen möchtest");
            return;
        }
    };

    let task = match Task::get_task(task_name) {
        Some(task) => task,
        None => {
            log_warning!("Task nicht gefunden ");
            return;
        }
    };
    // delete the task
    task.delete_as_file();
    log_info!("Task erfolgreich gelöscht");
}

fn list() {
    log_info!("--------> Tasks <--------");
    for task in Task::get_task_all() {
        log_info!("{}", task.get_name());
    }
}
