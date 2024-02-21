use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::core::task::Task;
use crate::utils::logger::Logger;
use crate::{log_error, log_info};

#[derive(Serialize, Deserialize)]
pub struct ServiceNumber {
    service_number: u32,
}

pub struct ApiGet;

impl ApiGet {
    pub async fn task(path: web::Path<String>) -> HttpResponse {
        let task_name = path.into_inner();

        log_info!("get Task Name {}", task_name);

        let task = match Task::get_task(task_name) {
            Some(task) => task,
            None => {
                return HttpResponse::NoContent().finish();
            }
        };

        log_info!("Task objekt {}", task.get_name());

        return match task.to_json() {
            Some(data) => HttpResponse::Ok().json(data),
            None => return HttpResponse::NoContent().finish(),
        };
    }

    pub async fn services() -> HttpResponse {
        let service_number = ServiceNumber { service_number: 1 };

        let json = match serde_json::to_value(&service_number) {
            Ok(json) => json,
            Err(e) => {
                log_error!("{}", e.to_string());
                return empty_json_response();
            }
        };
        HttpResponse::Ok().json(json)
    }
}

pub fn empty_json_response() -> HttpResponse {
    let empty_json = serde_json::json!({});
    HttpResponse::Ok().json(empty_json)
}
