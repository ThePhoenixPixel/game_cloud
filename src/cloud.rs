use crate::lib::bx::Bx;
use crate::logger::Logger;
use crate::sys_config::cloud_config::CloudConfig;
use crate::sys_config::software_config::SoftwareConfig;
use colored::Colorize;
use std::env;
use std::fmt::format;
use std::path::PathBuf;

pub struct Cloud;

impl Cloud {
    pub fn enable() {
        // print the logo
        Cloud::print_icon();

        //check the cloud config.json
        CloudConfig::check();

        // check folder
        Cloud::check_folder();

        // check software config file
        SoftwareConfig::check();

        // check the software files
        Cloud::check_software();
    }

    pub fn disable() {}

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
        println!("_____{}__________________________________________________________{}__{}________________________________________{}_____", r"/\\\\\\\\\\\\".red(), r"/\\\\\\\\\".cyan(), r"/\\\\\\".cyan(), r"/\\\".cyan() );
        println!("___{}________________________________________________________{}__{}_______________________________________{}_____", r"/\\\//////////".red(), r"/\\\////////".cyan(), r"\////\\\".cyan(), r"\/\\\".cyan() );
        println!("__{}_________________________________________________________________{}______________{}_______________________________________{}_____", r"/\\\".red(), r"/\\\/".cyan(), r"\/\\\".cyan(), r"\/\\\".cyan() );
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

    pub fn check_software() {
        let software_types = SoftwareConfig::get().get_software_types();

        for (software_type_name, software_type) in software_types {
            let software_path = CloudConfig::get()
                .get_cloud_path()
                .get_system_folder()
                .get_software_files_folder_path()
                .join(&software_type_name);
            Bx::create_path(&software_path);

            for software in software_type.get_software_names() {
                match Bx::download_file(software.get_download().as_str(), &software_path) {
                    Ok(_) => {
                        Logger::info(
                            format!(
                                "Successfully download the Software from url {}",
                                software.get_download()
                            )
                            .as_str(),
                        );
                    }
                    Err(e) => {
                        Logger::error(&e.to_string());
                        panic!("Can not download the software {}", software.get_download());
                    }
                }
            }
        }
    }
}
