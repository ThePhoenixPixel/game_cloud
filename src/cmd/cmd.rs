use std::io;
use std::io::Write;
use colored::ColoredString;
use crate::cmd::command::cmd_stop::CmdStop;
use crate::cmd::command::cmd_task::CmdTask;
use crate::cmd::command::cmd_template::CmdTemplate;
use crate::config::Config;


pub struct Cmd {
    pub command: String,
    pub args: Vec<String>,
    pub prefix: ColoredString,
}

impl Cmd {
    pub fn new() -> Cmd {
        let command = String::new();
        let args:Vec<String> = Vec::new();
        let prefix: ColoredString = ColoredString::default();
        Cmd {
            command,
            args,
            prefix
        }
    }

    pub fn set_prefix(&mut self, prefix: ColoredString){
        self.prefix = prefix;
    }

    pub fn start(&mut self){
        loop {

            print!("{} ", Config::get_prefix());
            io::stdout().flush().expect("Fehler beim Flushen des Puffers");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Fehler beim Lesen der Eingabe");

            // Teile die Eingabe in Teile auf
            let mut all_args: Vec<&str> = input.trim().split_whitespace().collect();

            if let Some(command) = all_args.first() {
                self.command = command.to_string();
                all_args.remove(0);
            } else {
                self.command.clear(); // Falls keine Teile vorhanden sind, leere den Befehl.
            }

            // Der Rest sind die Argumente
            self.args = all_args.iter().map(|s| s.to_string()).collect();

            if input.trim() == "exit" {
                break;
            } else {
                if self.process() {
                    break;
                }
            }

        }
    }

    pub fn process(&self) -> bool{
        match self.command.as_str() {
            "task" => CmdTask::execute(&self.args),

            "template" => CmdTemplate::execute(&self.args),

            "stop" => return CmdStop::execute(&self.args),

            _ => {
                println!("{} Unbekannter Befehl: {}", Config::get_prefix(), self.command);
                println!("{} Benutze task / stop / template", Config::get_prefix());
            }
        }
        return false;
    }
}