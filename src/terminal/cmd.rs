use colored::{ColoredString, Colorize};
use std::io;
use std::io::Write;

use crate::cloud::Cloud;
use crate::log_error;
use crate::terminal::command::cmd_help::CmdHelp;
use crate::terminal::command::cmd_service::CmdService;
use crate::terminal::command::cmd_task::CmdTask;
use crate::terminal::command::cmd_template::CmdTemplate;
use crate::terminal::command_manager::CommandManager;
use crate::utils::logger::Logger;

pub struct Cmd {
    prefix: ColoredString,
}

impl Cmd {
    pub fn new(prefix: &ColoredString) -> Cmd {
        Cmd {
            prefix: prefix.clone(),
        }
    }

    pub fn start(&self) {
        //start the cmd system
        loop {
            // print the prefix
            print!(
                "{} ",
                ColoredString::from(format!("{} >>", &self.prefix)).blue()
            );

            // flush the buffer
            flush_buffer();

            // read line input from terminal
            let input = read_from_line();

            // trim the input
            let args: Vec<&str> = input.trim().split_whitespace().collect();

            let command = match args.first() {
                Some(command) => command.to_string(),
                None => String::new(),
            }
            .to_lowercase();

            // check ob stop or exit execute in the input the stop the cloud
            if command == "exit" || command == "stop" {
                break;
            }

            // execute the commands
            match Cmd::execute_command(command.as_str(), args) {
                Ok(_) => {}
                Err(e) => log_error!("{}", e),
            }
        }
        Cloud::disable();
    }

    pub fn execute_command(command: &str, args: Vec<&str>) -> Result<(), String> {
        return match command {
            "help" => CmdHelp::execute(args),
            "task" => CmdTask::execute(args),
            "service" => CmdService::execute(args),
            "template" => CmdTemplate::execute(args),
            "" => Ok(()),
            _ => Err("Bitte gebe ein gültigen command an".to_string()),
        };
    }
}

fn flush_buffer() {
    match io::stdout().flush() {
        Ok(_) => return,
        Err(e) => {
            log_error!("Error by flushing the Buffer");
            log_error!("{}", e.to_string());
        }
    }
}

fn read_from_line() -> String {
    let mut input = String::new();
    return match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(e) => {
            log_error!("Error by read the input");
            log_error!("{}", e.to_string());
            String::new()
        }
    };
}
