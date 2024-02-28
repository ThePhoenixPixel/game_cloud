use crate::core::installer::Installer;
use crate::core::software::Software;
use crate::core::task::Task;
use crate::core::template::Template;
use crate::sys_config::software_config::SoftwareConfig;
use crate::terminal::command_manager::CommandManager;
use crate::utils::logger::Logger;
use crate::{log_error, log_info, log_warning};

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
            "create" => create(args),
            "remove" => remove(args),
            "list" => list(),
            "info" => info(args),
            "setup" => setup(args),
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

fn setup(args: Vec<&str>) {
    // command task setup <name> <set / add / remove / clear> <task_artibut> <new_wert>
    let task_name = match args.get(2) {
        Some(task_name) => task_name,
        None => {
            log_warning!("Bitte gebe ein Task namen an den du bearbeiten möchtest");
            return;
        }
    };

    let task = match Task::get_task(task_name.to_string()) {
        Some(task) => task,
        None => {
            log_warning!("Bitte gebe ein task an der exsistiert");
            return;
        }
    };

    let was_wilste_machen = match args.get(3) {
        Some(wert) => wert,
        None => {
            log_warning!(
                "Bitte gebe ein WErt an was du machen möchtest add / set / remove / clear"
            );
            return;
        }
    };

    let task_atribut = match args.get(4) {
        Some(task_atribut) => task_atribut,
        None => {
            log_warning!(
                "Bitte gebe task atribut an welches du verändern möchtest zb split oder den ram"
            );
            return;
        }
    };

    match was_wilste_machen.to_lowercase().as_str() {
        "add" => {
            setup_add(task, task_atribut, &args);
        }
        "set" => {
            setup_set(task, task_atribut, &args);
        }
        "remove" => {
            setup_remove(task, task_atribut, &args);
        }
        "clear" => {
            setup_clear(task, task_atribut);
        }
        _ => {
            log_warning!(
                "Dies ist kein Gültiges argument verwende eins davon / add / set / remove / clear",
            );
            return;
        }
    }
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

fn setup_remove(mut task: Task, attribute: &str, args: &Vec<&str>) {
    let wert = match args.get(5) {
        Some(new_wert) => new_wert,
        None => {
            log_warning!("Bitte gebe ein neuen wert an");
            return;
        }
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
                None => {
                    log_warning!("Bitte gebe ein template namen an");
                    return;
                }
            };

            let mut template = Template::new();
            template.set_template(&wert.to_string());
            template.set_name(&template_name.to_string());

            task.remove_template(template)
        }
        _ => {
            log_warning!("Bitte gebe ein gültigen atribut Wert an");
            return;
        }
    }
}
fn setup_set(mut task: Task, attribute: &str, args: &Vec<&str>) {
    let new_wert = match args.get(5) {
        Some(new_wert) => new_wert,
        None => {
            log_warning!("Bitte gebe ein neuen wert an");
            return;
        }
    };

    match attribute {
        "name" => {
            task.change_name(new_wert.to_string());
            log_info!("Task name erfolgreich geändert");
        }
        "split" => {
            let new_wert: char = match new_wert.as_bytes().get(0).copied() {
                Some(byte) => byte as char,
                None => {
                    log_warning!("Bitte gebe als split carakter nur ein zeichen an");
                    return;
                }
            };
            task.set_split(&new_wert);
            log_info!("Split wurde geändert");
        }
        "delete_on_stop" => {
            let delete_on_stop: bool = match new_wert.parse() {
                Ok(delete_on_stop) => delete_on_stop,
                Err(e) => {
                    log_warning!("Bitte gebe true oder false nur an");
                    log_error!("{}", e.to_string());
                    return;
                }
            };
            task.set_delete_on_stop(delete_on_stop);
            log_info!("delete_on_stop wurde geändert");
        }
        "static_service" => {
            let static_service: bool = match new_wert.parse() {
                Ok(static_service) => static_service,
                Err(e) => {
                    log_warning!("Bitte gebe true oder false nur an");
                    log_error!("{}", e.to_string());
                    return;
                }
            };
            task.set_static_service(static_service);
            log_info!("static_service wurde geändert");
        }
        "software" => {
            let software_name = match args.get(5) {
                Some(new_wert) => new_wert,
                None => {
                    log_warning!("Bitte gebe ein neuen wert an");
                    return;
                }
            };

            let software_type = match SoftwareConfig::get().get_software_type(new_wert) {
                Some(software_type) => software_type,
                None => {
                    log_warning!("Bitte gebe ein Software Type an der exsistiert");
                    return;
                }
            };

            let software_name = match software_type.get_software_name(software_name) {
                Some(software) => software,
                None => {
                    log_warning!("");
                    return;
                }
            };

            let software = Software::new(&new_wert.to_string(), &software_name.get_name());
            task.set_software(software);
            log_info!("Software erfolgreich gesetzt");
        }
        "max_ram" => {
            let max_ram: u32 = match new_wert.parse() {
                Ok(n) => n,
                Err(e) => {
                    log_warning!("Bitte gebe eine ganze Zahl an");
                    log_error!("{}", e.to_string());
                    return;
                }
            };
            task.set_max_ram(&max_ram);
            log_info!("Max Ram wurde geändert");
        }
        "start_port" => {
            let start_port: u32 = match new_wert.parse() {
                Ok(start_port) => start_port,
                Err(e) => {
                    log_warning!("Bitte gebe eine ganze Zahl an");
                    log_error!("{}", e.to_string());
                    return;
                }
            };
            task.set_start_port(start_port);
            log_info!("Start Port wurde geändert");
        }
        "min_service_count" => {
            let min_service_count: u32 = match new_wert.parse() {
                Ok(min_service_count) => min_service_count,
                Err(e) => {
                    log_warning!("Bitte gebe eine ganze Zahl an");
                    log_error!("{}", e.to_string());
                    return;
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
        _ => {
            log_warning!("Bitte gebe ein gültigen atribut Wert an");
            return;
        }
    }
}

fn setup_add(mut task: Task, attribute: &str, args: &Vec<&str>) {
    let new_wert = match args.get(5) {
        Some(new_wert) => new_wert,
        None => {
            log_warning!("Bitte gebe ein neuen wert an");
            return;
        }
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
            let mut template = Template::new();
            template.set_template(&new_wert.to_string());

            log_warning!("Noch nicht implementiert hier wird args used");
        }
        _ => {
            log_warning!("Bitte gebe ein gültigen atribut Wert an");
            return;
        }
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
    log_info!("--------------------------");
}
