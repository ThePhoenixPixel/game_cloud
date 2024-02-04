use crate::data::task::Task;
use crate::logger::Logger;
use crate::rest_api::get::ApiGet;
use crate::sys_config::cloud_config::CloudConfig;
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};

pub struct ApiMain;

impl ApiMain {
    #[actix_web::main]
    pub async fn start() {
        Logger::info("Start the REST AIP Server");

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
                Logger::warning(
                    format!(
                        "Can not bind the REST API Server at {}",
                        CloudConfig::get().get_rest_api().to_string()
                    )
                    .as_str(),
                );
                Logger::error(e.to_string().as_str());
                return;
            }
        };

        // start the server
        match http_server.run().await {
            Ok(_) => Logger::info("Rest Api Server successfully start"),
            Err(e) => {
                Logger::error(e.to_string().as_str());
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

    Logger::info(format!("get Task Name {}", task_name).as_str());

    let task = match Task::get_task(task_name) {
        Some(task) => task,
        None => {
            return HttpResponse::NoContent().finish();
        }
    };

    Logger::info(format!("task objekt {}", task.get_name()).as_str());

    return match task.to_json() {
        Some(data) => HttpResponse::Ok().json(data),
        None => return HttpResponse::NoContent().finish(),
    };
}
