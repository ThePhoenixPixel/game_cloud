use crate::core::installer::Installer;
use crate::core::software::Software;
use crate::core::task::Task;
use crate::core::template::Template;
use crate::sys_config::software_config::SoftwareConfig;
use crate::terminal::command_manager::CommandManager;
use crate::utils::logger::Logger;
use crate::{log_info, log_warning};

pub struct CmdTask;

impl CommandManager for CmdTask {
    async fn execute(args: Vec<&str>) -> Result<(), String> {
        // get the first argument command task <arg1>
        let arg1 = match args.get(1) {
            Some(arg) => *arg,
            None => return Err("Bitte gebe ein ein der volgebedej  argumente an".to_string()),
        };

        match arg1 {
            "create" => Ok(create(args)),
            "delete" => Ok(delete(args)),
            "list" => Ok(list()),
            "info" => Ok(info(args)),
            "setup" => setup(args),
            "reload" => Ok(Task::reload().await),
            _ => Err(
                "Dies ist kein Gültiges argument verwende eins davon / add / remove / list"
                    .to_string(),
            ),
        }
    }
    fn tab_complete(_args: Vec<&str>) -> Vec<String> {
        todo!()
    }
}

fn setup(args: Vec<&str>) -> Result<(), String> {
    // command task setup <name> <set / add / remove / clear> <task_artibut> <new_wert>
    let task_name = match args.get(2) {
        Some(task_name) => task_name,
        None => return Err("Bitte gebe ein Task namen an den du bearbeiten möchtest".to_string()),
    };

    let task = match Task::get_task(task_name.to_string()) {
        Some(task) => task,
        None => return Err("Bitte gebe ein task an der exsistiert".to_string()),
    };

    let was_wilste_machen = match args.get(3) {
        Some(wert) => wert,
        None => {
            return Err(
                "Bitte gebe ein WErt an was du machen möchtest add / set / remove / clear"
                    .to_string(),
            )
        }
    };

    let task_atribut =
        match args.get(4) {
            Some(task_atribut) => task_atribut,
            None => return Err(
                "Bitte gebe task atribut an welches du verändern möchtest zb split oder den ram"
                    .to_string(),
            ),
        };

    match was_wilste_machen.to_lowercase().as_str() {
        "add" => {
            setup_add(task, task_atribut, &args)?;
        }
        "set" => {
            setup_set(task, task_atribut, &args)?;
        }
        "remove" => {
            setup_remove(task, task_atribut, &args)?;
        }
        "clear" => {
            setup_clear(task, task_atribut);
        }
        _ => {
            return Err(
                "Dies ist kein Gültiges argument verwende eins davon / add / set / remove / clear"
                    .to_string(),
            )
        }
    }
    Ok(())
}

fn setup_clear(mut task: Task, attribute: &str) {
    match attribute {
        "node" => {
            task.clear_nodes();
            log_info!("Erfoldgreich alle Nodes cleart");
        }
        "group" => {
            task.clear_groups();
            log_info!("Erfoldgreich alle Groups cleart");
        }
        "template" => {
            task.clear_templates();
            log_warning!("Alle Templates Gelöscht");
        }
        _ => {
            log_warning!("Bitte gebe ein gültigen atribut Wert an");
            return;
        }
    }
}

fn setup_remove(mut task: Task, attribute: &str, args: &Vec<&str>) -> Result<(), String> {
    let wert = match args.get(5) {
        Some(new_wert) => new_wert,
        None => return Err("Bitte gebe ein neuen wert an".to_string()),
    };
    match attribute {
        "node" => {
            task.remove_node(&wert.to_string());
        }
        "group" => {
            task.remove_group(&wert.to_string());
        }
        "template" => {
            let template_name = match args.get(6) {
                Some(template_name) => template_name,
                None => return Err("Bitte gebe ein template namen an".to_string()),
            };

            let mut template = Template::new();
            template.set_template(&wert.to_string());
            template.set_name(&template_name.to_string());

            task.remove_template(template)
        }
        _ => return Err("Bitte gebe ein gültigen atribut Wert an".to_string()),
    }
    Ok(())
}
fn setup_set(mut task: Task, attribute: &str, args: &Vec<&str>) -> Result<(), String> {
    let new_wert = match args.get(5) {
        Some(new_wert) => new_wert,
        None => return Err("Bitte gebe ein neuen wert an".to_string()),
    };

    match attribute {
        "name" => {
            task.change_name(new_wert.to_string());
            log_info!("Task name erfolgreich geändert");
        }
        "split" => {
            let new_wert: char = match new_wert.as_bytes().get(0).copied() {
                Some(byte) => byte as char,
                None => return Err("Bitte gebe als split carakter nur ein zeichen an".to_string()),
            };
            task.set_split(&new_wert);
            log_info!("Split wurde geändert");
        }
        "delete_on_stop" => {
            let delete_on_stop: bool = match new_wert.parse() {
                Ok(delete_on_stop) => delete_on_stop,
                Err(e) => {
                    return Err(format!(
                        "Bitte gebe true oder false nur an \n {}",
                        e.to_string()
                    ))
                }
            };
            task.set_delete_on_stop(delete_on_stop);
            log_info!("delete_on_stop wurde geändert");
        }
        "static_service" => {
            let static_service: bool = match new_wert.parse() {
                Ok(static_service) => static_service,
                Err(e) => {
                    return Err(format!(
                        "Bitte gebe true oder false nur an \n {}",
                        e.to_string()
                    ))
                }
            };
            task.set_static_service(static_service);
            log_info!("static_service wurde geändert");
        }
        "software" => {
            let software_name = match args.get(5) {
                Some(new_wert) => new_wert,
                None => return Err("Bitte gebe ein neuen wert an".to_string()),
            };

            let software_type = match SoftwareConfig::get().get_software_type(new_wert) {
                Some(software_type) => software_type,
                None => return Err("Bitte gebe ein Software Type an der exsistiert".to_string()),
            };

            let software_name = match software_type.get_software_name(software_name) {
                Some(software) => software,
                None => return Err("Bitte gebe ein Software Namen an der exsistiert".to_string()),
            };

            let software = Software::new(&new_wert.to_string(), &software_name.get_name());
            task.set_software(software);
            log_info!("Software erfolgreich gesetzt");
        }
        "max_ram" => {
            let max_ram: u32 = match new_wert.parse() {
                Ok(n) => n,
                Err(e) => {
                    return Err(format!(
                        "Bitte gebe eine ganze Zahl an \n {}",
                        e.to_string()
                    ))
                }
            };
            task.set_max_ram(&max_ram);
            log_info!("Max Ram wurde geändert");
        }
        "start_port" => {
            let start_port: u32 = match new_wert.parse() {
                Ok(start_port) => start_port,
                Err(e) => {
                    return Err(format!(
                        "Bitte gebe eine ganze zahl an \n {}",
                        e.to_string()
                    ))
                }
            };
            task.set_start_port(start_port);
            log_info!("Start Port wurde geändert");
        }
        "min_service_count" => {
            let min_service_count: u32 = match new_wert.parse() {
                Ok(min_service_count) => min_service_count,
                Err(e) => {
                    return Err(format!(
                        "Bitte gebe eine ganze Zahl an \n {}",
                        e.to_string()
                    ))
                }
            };
            task.set_min_service_count(min_service_count.clone());
            println!("{}", min_service_count);
            log_info!("min_service_count wurde geändert");
        }
        "installer" => {
            task.set_installer(&Installer::from(new_wert));
            log_info!("Installer erfolgreich gesetzt");
        }
        _ => return Err("Bitte gebe ein gültigen atribut Wert an".to_string()),
    }
    Ok(())
}

fn setup_add(mut task: Task, attribute: &str, args: &Vec<&str>) -> Result<(), String> {
    let new_wert = match args.get(5) {
        Some(new_wert) => new_wert,
        None => return Err("Bitte gebe ein neuen wert an".to_string()),
    };

    match attribute {
        "node" => {
            task.add_node(new_wert.to_string());
            log_info!("Erfoldgreich den Node hinzugefügt");
        }
        "group" => {
            task.add_group(&new_wert.to_string());
            log_info!("Group erfolgreich hinzugefügt");
        }
        "template" => {
            let template_name = match args.get(6) {
                Some(template_name) => template_name,
                None => return Err("Bitte gebe ein template Namen an".to_string()),
            };

            let template_priority_str = match args.get(7) {
                Some(template_priority) => template_priority,
                None => return Err("Bitte gebe eine Template Priority an".to_string()),
            };

            let template_priority: u32 = match template_priority_str.parse() {
                Ok(prio) => prio,
                Err(_) => return Err("Bitte gebe eine ganze Zahl an".to_string()),
            };

            task.add_template(Template::create(
                &new_wert.to_string(),
                &template_name.to_string(),
                &template_priority,
            ));
            log_warning!("Template erfolgreich hinzugefügt");
        }
        _ => return Err("Bitte gebe ein gültigen attribut Wert an".to_string()),
    }
    return Ok(());
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

    match Task::get_task(task_name) {
        Some(task) => task.print(),
        None => {
            log_warning!("Bitte gebe ein task namen an der exsistiert");
            return;
        }
    };
}

fn create(args: Vec<&str>) {
    // command: task create <name> <software_type> <software_name>
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

    Task::create(&task_name, &Software::new(&software_type, &software_name));
    log_info!("Task Erfolgreich erstellt");
}

fn delete(args: Vec<&str>) {
    // command task remove <name>

    let task_name = match args.get(2) {
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
    log_info!("--------------------------");
}
