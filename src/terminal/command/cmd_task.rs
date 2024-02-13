use crate::logger::Logger;
use crate::terminal::command_manager::CommandManager;

pub struct CmdTask;

impl CommandManager for CmdTask {
    fn execute(args: Vec<&str>) {
        Logger::info("in task cmd ");
        println!("{:?}", args);
    }

    fn tab_complete(args: Vec<&str>) -> Vec<String> {
        todo!()
    }
}
