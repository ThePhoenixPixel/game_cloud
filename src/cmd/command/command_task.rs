use crate::cmd::command_manager::Command;
use crate::language::Language;

pub struct CommandTask;

impl Command for CommandTask {
    fn execute(&self, args: Vec<String>) {
        if args.is_empty() {
            println!("MyCommand ausgeführt ohne Argumente");
        } else {
            println!("{}", Language::translate("cmd.command.args.empty"));
        }
    }

    fn tab_complete(&self, input: &str) -> Vec<String> {
        let subcommands = vec!["subcommand1", "subcommand2"];
        subcommands
            .into_iter()
            .filter(|&cmd| cmd.starts_with(input))
            .map(|cmd| cmd.to_string())
            .collect()
    }
}
fn add(args: Vec<String>) {

}
