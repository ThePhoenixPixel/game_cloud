use crate::{log_error, log_info, log_warning};
use crate::core::network::node_get::NodeGet;
use crate::sys_config::cloud_config::CloudConfig;
use crate::utils::logger::Logger;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use crate::core::network::node_post::NodePost;

pub struct NodeHost;

impl NodeHost {
    #[actix_web::main]
    pub async fn start() {
        log_info!("Start the Node Host");
        let app_factory = || {
            App::new()
                .wrap(
                    Cors::permissive()
                        .allow_any_method()
                        .supports_credentials()
                        .allow_any_header(),
                )
                .service(web::resource("cloud/api/get/onlineService").route(web::get().to(NodeGet::online_service)))
                .service(web::resource("cloud/api/service/setOnlineMode").route(web::post().to(NodePost::set_online_mode)))
        };

        // bind the address
        let http_server = match HttpServer::new(app_factory)
            .bind(CloudConfig::get().get_node_host().to_string())
        {
            Ok(http_server) => http_server,
            Err(e) => {
                log_warning!(
                    "Can not bind the NODE Server at {}",
                    CloudConfig::get().get_node_host().to_string()
                );
                log_error!("{}", e.to_string());
                return;
            }
        };

        // start the server
        match http_server.run().await {
            Ok(_) => log_info!("Node Server successfully start"),
            Err(e) => {
                log_error!("{}", e.to_string());
                return;
            }
        }
    }
}