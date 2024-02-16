use crate::terminal::command_manager::CommandManager;

pub struct CmdHelp;

impl CommandManager for CmdHelp {
    fn execute(args: Vec<&str>) {
        todo!()
    }

    fn tab_complete(args: Vec<&str>) -> Vec<String> {
        todo!()
    }
}
