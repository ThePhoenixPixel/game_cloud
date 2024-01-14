use std::collections::HashMap;

pub trait Command {
    fn execute(&self, args: Vec<String>);
    fn tab_complete(&self, input: &str) -> Vec<String>;
}

pub struct CommandManager {
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandManager {
    pub fn new() -> Self {
        CommandManager {
            commands: HashMap::new(),
        }
    }

    pub fn register_command(&mut self, name: &str, command: Box<dyn Command>) {
        self.commands.insert(name.to_string(), command);
    }

    pub fn execute_command(&self, name: &str, args: Vec<String>) {
        if let Some(command) = self.commands.get(name) {
            command.execute(args);
        } else {
            println!("Unbekannter Befehl: {}", name);
        }
    }

    pub fn tab_complete(&self, input: &str) -> Vec<String> {
        let mut completions = Vec::new();
        for command in self.commands.values() {
            completions.extend(command.tab_complete(input).into_iter());
        }
        completions
    }
}
