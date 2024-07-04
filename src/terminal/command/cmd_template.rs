use crate::terminal::command_manager::CommandManager;

pub struct CmdTemplate;

impl CommandManager for CmdTemplate {
    async fn execute(_args: Vec<&str>) -> Result<(), String> {
        todo!()
    }

    fn tab_complete(_args: Vec<&str>) -> Vec<String> {
        todo!()
    }
}
