use crate::config::Config;

pub struct CmdStop;

impl CmdStop {
    pub fn execute(args: &Vec<String>) -> bool{

        CmdStop::shutdown_all_service();
        true
    }
    fn shutdown_all_service(){


        println!("{} All Service Closed", Config::get_prefix());
    }
}


