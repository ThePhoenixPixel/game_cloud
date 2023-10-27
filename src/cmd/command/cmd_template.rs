use crate::config::Config;
use crate::data::template::Template;

pub struct CmdTemplate;

impl CmdTemplate {
    pub fn execute(args: &Vec<String>){

        if let Some(arg0) = args.get(0){

            match arg0.as_str() {

                "list" => {

                }

                "create" => {
                    CmdTemplate::create(args);
                }

                _ => {
                    println!("{} Kein Gültiges Argument", Config::get_prefix());
                }
            }
        } else {
            println!("bitte gebe ein Argument an");
        }


    }

    fn create(args: &Vec<String>){
        if let Some(template_task) = args.get(1) {
            if let Some(template_name) = args.get(2) {
                //new template obj
                let mut template = Template::new();
                template.set_template(template_task);
                template.set_name(template_name);
                template.create_by_self();
                println!("{} Template erfolgreich erstellt", Config::get_prefix());

            } else {
                println!("{} Bitte gebe einen namen für das unter Template an", Config::get_prefix());
            }
        } else {
            println!("{} Bitte gebe ein Template namen an", Config::get_prefix());
        }
    }
}