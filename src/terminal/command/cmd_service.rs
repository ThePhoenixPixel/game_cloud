use crate::core::service::Service;
use crate::log_info;
use crate::utils::logger::Logger;
use crate::terminal::command_manager::CommandManager;

pub struct CmdService;

impl CommandManager for CmdService {
    fn execute(args: Vec<&str>) -> Result<(), String> {
        let arg1 = match args.get(1) {
            Some(arg1) => *arg1,
            None => return Err("bitte gebe ein argument an -> list /  an".to_string()),
        };

        return match arg1 {
            "list" => list(args),
            _ => {
                Err("bitte gebe ein gültiges argument an -> list /  an".to_string())
            }
        };
    }
    fn tab_complete(_args: Vec<&str>) -> Vec<String> {
        todo!()
    }
}

fn list(args: Vec<&str>) -> Result<(), String> {
    let arg2 = match args.get(2) {
        Some(arg2) => *arg2,
        None => return list_all(),
    };

    return match arg2 {
        "--online" => list_online(),
        "--on" => list_online(),
        _ => Err("bitte gebe einen gültigen para an -> --online, --on".to_string()),
    };
}

fn list_online() -> Result<(), String> {
    let services = Service::get_all_service();
    log_info!("Dies sind alle Online Services:");
    log_info!("Name | Server Address | Plugin Listener");
    for service in services {
        if !service.is_start() {
            return Ok(());
        }

        log_info!("{} | {} | {}", service.get_name(), service.get_server_address().to_string(), service.get_plugin_listener().to_string());
    }
    Ok(())
}

fn list_all() -> Result<(), String> {
    let services = Service::get_all_service();
    log_info!("Name | Server Address | Plugin Listener | Online");
    for service in services {
        log_info!("{} | {} | {} | {} ", service.get_name(), service.get_server_address().to_string(), service.get_plugin_listener().to_string(), service.is_start());
    }
    Ok(())
}