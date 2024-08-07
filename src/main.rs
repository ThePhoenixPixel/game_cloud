use crate::cloud::Cloud;

#[cfg(feature = "rest-api")]
pub mod rest_api {
    pub mod api_main;
    pub mod get;
    pub mod set;
}

pub mod utils {
    pub mod log;
    pub mod logger;
    pub mod service_status;
    #[macro_use]
    pub mod logger_macros;
}

pub mod core {
    pub mod network {
        pub mod requests {
            pub mod register_server;
            pub mod shutdown;
        }
        pub mod node_host;
        pub mod node_get;
        pub mod node_post;
    }

    pub mod task;
    pub mod template;
    //pub mod group;
    //pub mod node;
    pub mod installer;
    pub mod service;
    pub mod software;
}

pub mod sys_config {
    pub mod cloud_config;
    pub mod software_config;
}

pub mod terminal {
    pub mod cmd;
    pub mod command_manager;

    pub mod command {
        pub mod cmd_help;
        pub mod cmd_me;
        pub mod cmd_service;
        pub mod cmd_task;
        pub mod cmd_template;
    }
}

pub mod cloud;
pub mod language;

const VERSION: &str = "0.1";

#[tokio::main]
async fn main() {
    println!("Start Game Cloud...");
    //start the game cloud
    Cloud::enable(VERSION).await;

    //disable the game cloud
    Cloud::disable();

    println!("Game Cloud Stop");
    println!("Good Bye");
}
