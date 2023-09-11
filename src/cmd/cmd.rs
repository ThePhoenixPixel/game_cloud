use std::env::args;
use std::io;
use std::io::Write;
use crate::cmd::command::cmd_stop::execute_stop;
use crate::cmd::command::cmd_task::{CmdTask, execute_task};
use crate::config::Config;


pub struct Cmd {
    pub command: String,
    pub args: Vec<String>,
}

impl Cmd {
    pub fn new() -> Cmd {
        Cmd {
            command,
            args
        }
    }

    pub fn start(&self){
        loop {

            print!("{} ", Config::get_prefix());
            io::stdout().flush().expect("Fehler beim Flushen des Puffers");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Fehler beim Lesen der Eingabe");

            // Verarbeite die Eingabe
            let command = input.trim();
            let args: Vec<&str> = input.split_whitespace().collect();
            if input.trim() == "exit" {
                break;
            } else {
                println!("Du hast eingegeben: {}", input);
            }

        }
    }

    pub fn process(&self) {
        match self.command.as_str() {
            "stop" => CmdTask::execute(&self.args),
            "task" => execute_stop(&self.args),
            _ => {
                println!("Unbekannter Befehl: {}", self.command);

            }
        }
    }
}