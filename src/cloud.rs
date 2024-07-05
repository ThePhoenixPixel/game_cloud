use std::time::Duration;
use colored::{ColoredString, Colorize};
use std::env;
use std::path::PathBuf;

use crate::lib::bx::Bx;
use crate::sys_config::cloud_config::CloudConfig;
use crate::sys_config::software_config::SoftwareConfig;
use crate::terminal::cmd::Cmd;
use crate::utils::logger::Logger;
use crate::{log_error, log_info};
use crate::core::network::node_host::NodeHost;

#[cfg(feature = "rest-api")]
use crate::rest_api::api_main::ApiMain;


pub struct Cloud;

impl Cloud {
    pub async fn enable(version: &str) {
        // download link
        let url = format!("http://download.codergames.de/minecloud/version/{}/", version);

        // print the logo
        Cloud::print_icon();

        //check the cloud config.json
        CloudConfig::check(&url).await;

        // check folder
        Cloud::check_folder();

        // check software config file
        SoftwareConfig::check(&url).await;

        // check the software files
        Cloud::check_software().await;

        // Cloud require system ist finish

        #[cfg(feature = "rest-api")]
        std::thread::spawn(move || {
            let _ = ApiMain::start();
        });

        tokio::time::sleep(Duration::from_secs(1)).await;

        std::thread::spawn(move || {
            let _ = NodeHost::start();
        });

        tokio::time::sleep(Duration::from_secs(1)).await;

        let cmd = Cmd::new(&ColoredString::from(CloudConfig::get().get_prefix().as_str()).cyan());
        cmd.start().await;
    }

    pub fn disable() {
        Cloud::shutdown_service();
        log_info!("Cloud shutdown");
        log_info!("Bye Bye");
        std::process::exit(0)
    }

    pub fn get_exe_path() -> PathBuf {
        return match env::current_exe() {
            Ok(mut exe) => {
                exe.pop();
                exe
            }
            Err(e) => {
                eprintln!("Error get the exe path");
                eprintln!("{}", e.to_string().as_str());
                panic!("The GameCloud has an fatal Error")
            }
        };
    }

    pub fn print_icon() {
        println!(" ");
        println!("_____{}__________________________________________________________{}__{}________________________________________{}_____", r"/\\\\\\\\\\\\".red(), r"/\\\\\\\\\".cyan(), r"/\\\\\\".cyan(), r"/\\\".cyan());
        println!("___{}________________________________________________________{}__{}_______________________________________{}_____", r"/\\\//////////".red(), r"/\\\////////".cyan(), r"\////\\\".cyan(), r"\/\\\".cyan());
        println!("__{}_________________________________________________________________{}______________{}_______________________________________{}_____", r"/\\\".red(), r"/\\\/".cyan(), r"\/\\\".cyan(), r"\/\\\".cyan());
        println!("_{}____{}__{}_______{}__{}_______{}___{}________________{}________{}_____{}____{}________{}_____", r"\/\\\".red(), r"/\\\\\\\".red(), r"/\\\\\\\\\".red(), r"/\\\\\".red(), r"/\\\\\".red(), r"/\\\\\\\\".red(), r"/\\\".cyan(), r"\/\\\".cyan(), r"/\\\\\".cyan(), r"/\\\".cyan(), r"/\\\".cyan(), r"\/\\\".cyan());
        println!(
            "_{}___{}_{}____{}___{}_{}________________{}______{}__{}___{}___{}_____",
            r"\/\\\".red(),
            r"\/////\\\".red(),
            r"\////////\\\".red(),
            r"/\\\///\\\\\///\\\".red(),
            r"/\\\/////\\\".red(),
            r"\/\\\".cyan(),
            r"\/\\\".cyan(),
            r"/\\\///\\\".cyan(),
            r"\/\\\".cyan(),
            r"\/\\\".cyan(),
            r"/\\\\\\\\\".cyan()
        );
        println!(
            "__{}_______{}___{}__{}_{}__{}__{}__{}_______________{}_____{}__{}_{}___{}__{}____",
            r"\/\\\".red(),
            r"\/\\\".red(),
            r"/\\\\\\\\\\".red(),
            r"\/\\\".red(),
            r"\//\\\".red(),
            r"\/\\\".red(),
            r"/\\\\\\\\\\\".red(),
            r"\//\\\".cyan(),
            r"\/\\\".cyan(),
            r"/\\\".cyan(),
            r"\//\\\".cyan(),
            r"\/\\\".cyan(),
            r"\/\\\".cyan(),
            r"/\\\////\\\".cyan()
        );
        println!(
            "___{}_______{}__{}__{}__{}__{}_{}____{}_____________{}____{}__{}__{}___{}_{}__{}___",
            r"\/\\\".red(),
            r"\/\\\".red(),
            r"/\\\/////\\\".red(),
            r"\/\\\".red(),
            r"\/\\\".red(),
            r"\/\\\".red(),
            r"\//\\///////".red(),
            r"\///\\\".cyan(),
            r"\/\\\".cyan(),
            r"\//\\\".cyan(),
            r"/\\\".cyan(),
            r"\/\\\".cyan(),
            r"\/\\\".cyan(),
            r"\/\\\".cyan(),
            r"\/\\\".cyan()
        );
        println!(
            "____{}__{}_{}__{}__{}__{}____{}__{}__{}___{}__{}_",
            r"\//\\\\\\\\\\\\/".red(),
            r"\//\\\\\\\\/\\".red(),
            r"\/\\\".red(),
            r"\/\\\".red(),
            r"\/\\\".red(),
            r"\//\\\\\\\\\\".red(),
            r"\////\\\\\\\\\".cyan(),
            r"/\\\\\\\\\".cyan(),
            r"\///\\\\\/".cyan(),
            r"\//\\\\\\\\\".cyan(),
            r"\//\\\\\\\/\\".cyan()
        );
        println!(
            "_____{}_____{}__{}___{}___{}____{}________{}__{}_____{}______{}____{}__",
            r"\////////////".red(),
            r"\////////\//".red(),
            r"\///".red(),
            r"\///".red(),
            r"\///".red(),
            r"\//////////".red(),
            r"\/////////".cyan(),
            r"\/////////".cyan(),
            r"\/////".cyan(),
            r"\/////////".cyan(),
            r"\///////\//".cyan()
        );
        println!(" ");
    }

    pub fn check_folder() {
        let config_path = CloudConfig::get().get_cloud_path();

        // create task folder
        Bx::create_path(&config_path.get_task_folder_path());

        // create template folder
        Bx::create_path(&config_path.get_template_folder_path());

        // create service temp folder
        Bx::create_path(&config_path.get_service_folder().get_temp_folder_path());

        // create service static folder
        Bx::create_path(&config_path.get_service_folder().get_static_folder_path());

        // create system plugin folder
        Bx::create_path(
            &config_path
                .get_system_folder()
                .get_system_plugins_folder_path(),
        );

        // create software files folder
        Bx::create_path(
            &config_path
                .get_system_folder()
                .get_software_files_folder_path(),
        );
    }

    pub async fn check_software() {
        let software_types = SoftwareConfig::get().get_software_types();

        for (software_type_name, software_type) in software_types {
            // create var paths
            let software_path = CloudConfig::get()
                .get_cloud_path()
                .get_system_folder()
                .get_software_files_folder_path()
                .join(&software_type_name);
            let system_plugins_path = CloudConfig::get()
                .get_cloud_path()
                .get_system_folder()
                .get_system_plugins_folder_path()
                .join(&software_type_name);

            // create the software types folder
            Bx::create_path(&software_path);
            Bx::create_path(&system_plugins_path);

            // iter to software types
            for software in software_type.get_software_names() {
                let system_plugins_path =
                    match Bx::extract_extension_from_url(&software.get_system_plugin().get_download()) {
                        Some(ext) => system_plugins_path.join(format!("MineCloud-{}.{}", software.get_name(), ext)),
                        None => system_plugins_path.join(software.get_name()),
                    };

                let software_path =
                    match Bx::extract_extension_from_url(&software.get_download().to_string()) {
                        Some(ext) => software_path.join(format!("{}.{}", software.get_name(), ext)),
                        None => software_path.join(software.get_name()),
                    };

                // download software when software file does not exist
                if !software_path.exists() {
                    log_info!("Download Software {}", software.get_name());
                    match Bx::download_file(software.get_download().as_str(), &software_path).await {
                        Ok(_) => {
                            log_info!(
                            "Successfully download the Software from url {}",
                            software.get_download()
                        );
                        }
                        Err(e) => {
                            log_error!("{}", e.to_string());
                            panic!("Can not download the software {}", software.get_download());
                        }
                    }
                }

                // download system plugin when plugin file does not exist
                if !software.get_system_plugin().is_local() && !system_plugins_path.exists() {
                    log_info!("Download Software System Plugin {} Plugin", software.get_name());
                    match Bx::download_file(software.get_system_plugin().get_download().as_str(), &system_plugins_path).await {
                        Ok(_) => {
                            log_info!(
                        "Successfully download the Software System Plugin from url {}",
                        software.get_system_plugin().get_download()
                    );
                        }
                        Err(e) => {
                            log_error!("{}", e.to_string());
                            panic!("Can not download the Software System Plugin {}", software.get_system_plugin().get_download());
                        }
                    }
                }
            }
        }
    }

    pub fn shutdown_service() {}
}
