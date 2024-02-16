use crate::terminal::command_manager::CommandManager;
use std::process::Command;

pub struct CmdTemplate;

impl CommandManager for CmdTemplate {
    fn execute(args: Vec<&str>) {
        todo!()
    }

    fn tab_complete(args: Vec<&str>) -> Vec<String> {
        todo!()
    }
}
