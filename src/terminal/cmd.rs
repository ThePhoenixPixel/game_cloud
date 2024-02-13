use crate::logger::Logger;
use crate::terminal::command::cmd_help::CmdHelp;
use crate::terminal::command::cmd_service::CmdService;
use crate::terminal::command::cmd_task::CmdTask;
use crate::terminal::command::cmd_template::CmdTemplate;
use crate::terminal::command_manager::CommandManager;

use crate::cloud::Cloud;
use colored::ColoredString;
use std::io;
use std::io::Write;

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
            print!("{} ", self.prefix);

            // flush the buffer
            flush_buffer();

            // read line input from terminal
            let input = read_from_line();

            // trim the input
            let mut args: Vec<&str> = input.trim().split_whitespace().collect();

            let command = match args.first() {
                Some(command) => command.to_string(),
                None => String::new(),
            }
            .to_lowercase();

            // remove the command arg from the args list
            args.remove(1);

            // check ob stop or exit execute in the input the stop the cloud
            if command == "exit" || command == "stop" {
                Cloud::disable();
                break;
            }

            // execute the commands
            Cmd::execute_command(command.as_str(), args)
        }
    }

    pub fn execute_command(command: &str, args: Vec<&str>) {
        Logger::info(command);
        match command {
            "help" => CmdHelp::execute(args),
            "task" => CmdTask::execute(args),
            "service" => CmdService::execute(args),
            "template" => CmdTemplate::execute(args),
            _ => {}
        }
    }
}

fn flush_buffer() {
    match io::stdout().flush() {
        Ok(_) => return,
        Err(e) => {
            Logger::error("Error by flushing the Buffer");
            Logger::error(e.to_string().as_str());
        }
    }
}

fn read_from_line() -> String {
    let mut input = String::new();
    return match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(e) => {
            Logger::error("Error by read the input");
            Logger::error(e.to_string().as_str());
            String::new()
        }
    };
}
