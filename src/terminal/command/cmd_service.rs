use crate::terminal::command_manager::CommandManager;

pub struct CmdService;

impl CommandManager for CmdService {
    fn execute(_args: Vec<&str>) -> Result<(), String> {
        todo!()
    }

    fn tab_complete(_args: Vec<&str>) -> Vec<String> {
        todo!()
    }
}
