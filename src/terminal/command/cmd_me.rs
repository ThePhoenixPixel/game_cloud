use psutil::process::Process;

use crate::log_info;
use crate::terminal::command_manager::CommandManager;
use crate::utils::logger::Logger;
pub struct CmdMe;

impl CommandManager for CmdMe {
    fn execute(_args: Vec<&str>) -> Result<(), String> {
        let pid = std::process::id();

        let mut process = match Process::new(pid) {
            Ok(process) => process,
            Err(e) => return Err(e.to_string()),
        };

        let cpu_percent = match process.cpu_percent() {
            Ok(cpu_percent) => cpu_percent,
            Err(e) => return Err(e.to_string()),
        };

        let ram_info = match process.memory_info() {
            Ok(ram_info) => ram_info,
            Err(e) => return Err(e.to_string()),
        };

        log_info!("------------>Cloud Info<------------");
        log_info!("Cpu: {:.2}%", cpu_percent);
        log_info!("Ram {} Bytes", ram_info.rss());
        log_info!("------------------------------------");

        Ok(())
    }

    fn tab_complete(_args: Vec<&str>) -> Vec<String> {
        todo!()
    }
}
