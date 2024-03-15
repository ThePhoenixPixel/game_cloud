use std::sync::Mutex;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Result, middleware, HttpRequest};

use crate::rest_api::get::ApiGet;
use crate::sys_config::cloud_config::CloudConfig;
use crate::utils::logger::Logger;
use crate::{log_error, log_info, log_warning};

pub struct AppState {
    token: String,
}

pub struct ApiMain;

impl ApiMain {
    #[actix_web::main]
    pub async fn start() {
        // init Zustand des Token
        let app_token = web::Data::new(Mutex::new(AppState {
            token: String::from("12345"),
        }));

        log_info!("Start the REST AIP Server");
        let app_factory = || {
            App::new()
                .app_data(app_token.clone()) // Teilen des Zustands zwischen Threads
                .wrap(middleware::DefaultHeaders::new().header("Content-Type", "application/json")) // Standardheader setzen
                .wrap_fn(|req, srv| validate_token(req, srv.data().clone())) // Verwendung der Closure
                .service(web::resource("cloud/get/task/{name}").route(web::get().to(ApiGet::task)))
        };

        // bind the address
        let http_server = match HttpServer::new(app_factory)
            .bind(CloudConfig::get().get_rest_api().to_string())
        {
            Ok(http_server) => http_server,
            Err(e) => {
                log_warning!(
                    "Can not bind the REST API Server at {}",
                    CloudConfig::get().get_rest_api().to_string()
                );
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
}

// Middleware-Funktion zur Überprüfung des Tokens
async fn validate_token(
    req: HttpRequest,
    data: web::Data<Mutex<AppState>>,
) -> Result<HttpRequest, actix_web::Error> {
    // Überprüfen, ob der Token in den Anfragedaten vorhanden ist
    match req.headers().get("Authorization") {
        Some(auth_header) => {
            if let Ok(auth_token) = auth_header.to_str() {
                if auth_token == &data.lock().unwrap().token {
                    Ok(req)
                } else {
                    Err(actix_web::error::ErrorUnauthorized("Invalid token!").into())
                }
            } else {
                Err(actix_web::error::ErrorUnauthorized("Invalid authorization header!").into())
            }
        }
        None => Err(actix_web::error::ErrorUnauthorized("Authorization header missing!").into()),
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


/*
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
*/
