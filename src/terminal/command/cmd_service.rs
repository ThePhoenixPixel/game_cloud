use crate::terminal::command_manager::CommandManager;

pub struct CmdService;

impl CommandManager for CmdService {
    fn execute(args: Vec<&str>) {
        todo!()
    }

    fn tab_complete(args: Vec<&str>) -> Vec<String> {
        todo!()
    }
}
