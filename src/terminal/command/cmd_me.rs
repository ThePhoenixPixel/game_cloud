use sysinfo::{Pid, System};

use crate::log_info;
use crate::terminal::command_manager::CommandManager;
use crate::utils::logger::Logger;

pub struct CmdMe;

impl CommandManager for CmdMe {
    async fn execute(_args: Vec<&str>) -> Result<(), String> {
        let pid = Pid::from_u32(std::process::id());

        match System::new().process(pid) {
            Some(process) => print_info(&process),
            None => return Err("Failed to get the System Info".to_string()),
        };

        Ok(())
    }

    fn tab_complete(_args: Vec<&str>) -> Vec<String> {
        todo!()
    }
}

fn print_info(process: &sysinfo::Process) {
    log_info!("------------>Cloud Info<------------");
    log_info!("Cpu: {:.2}%", process.cpu_usage());
    log_info!("Ram {} Bytes", process.memory());
    log_info!("------------------------------------");
}
