use crate::data::task::Task;
use crate::logger::Logger;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ServiceNumber {
    service_number: u32,
}

pub struct ApiGet;

impl ApiGet {
    pub async fn task(path: web::Path<String>) -> HttpResponse {
        let task_name = path.into_inner();

        log_info!(format!("get Task Name {}", task_name).as_str());

        let task = match Task::get_task(task_name) {
            Some(task) => task,
            None => {
                return HttpResponse::NoContent().finish();
            }
        };

        log_info!(format!("task objekt {}", task.get_name()).as_str());

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
                log_error!(e.to_string().as_str());
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
