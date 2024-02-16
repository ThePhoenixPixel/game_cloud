use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};

use crate::data::task::Task;
use crate::utils::logger::Logger;
use crate::rest_api::get::ApiGet;
use crate::sys_config::cloud_config::CloudConfig;
use crate::{log_error, log_info, log_warning};

pub struct ApiMain;

impl ApiMain {
    #[actix_web::main]
    pub async fn start() {
        log_info!("Start the REST AIP Server");
        let app_factory = || {
            App::new()
                .wrap(
                    Cors::permissive()
                        .allow_any_method()
                        .supports_credentials()
                        .allow_any_header(),
                )
                .service(web::resource("cloud/get/task/{name}").route(web::get().to(ApiGet::task)))
        };

        // bind the address
        let http_server = match HttpServer::new(app_factory)
            .bind(CloudConfig::get().get_rest_api().to_string())
        {
            Ok(http_server) => http_server,
            Err(e) => {
                log_warning!("Can not bind the REST API Server at {}", CloudConfig::get().get_rest_api().to_string());
                log_error!("{}", e.to_string());
                return;
            }
        };

        // start the server
        match http_server.run().await {
            Ok(_) => log_info!("Rest Api Server successfully start"),
            Err(e) => {
                log_error!("{}", e.to_string());
                return;
            }
        }
    }
    //alt
    /*
    HttpServer::new({
        App::new()
            .service(web::resource("/cloud/task/get/{name}").route(web::get().to(get_task)))
            //.service(web::resource("cloud/get/task/{name}").route(web::get().to(ApiGet::task)))
            .service(web::resource("cloud/get/services").route(web::get().to(ApiGet::services)))
    })
    .bind(CloudConfig::get().get_rest_api().to_string())
    .expect(&*format!("Can not bind the Rest Api Server at {}", CloudConfig::get().get_rest_api().to_string()))
    .run()
    .await
    .unwrap()
    */
}

async fn get_task(path: web::Path<(String)>) -> HttpResponse {
    let task_name = path.into_inner();

    log_info!("get Task Name {}", task_name);

    let task = match Task::get_task(task_name) {
        Some(task) => task,
        None => {
            return HttpResponse::NoContent().finish();
        }
    };

    log_info!("task objekt {}", task.get_name());

    return match task.to_json() {
        Some(data) => HttpResponse::Ok().json(data),
        None => return HttpResponse::NoContent().finish(),
    };
}
