use actix_web::{web, HttpResponse};

use crate::utils::logger::Logger;
use crate::log_error;


pub struct ApiSet;

impl ApiSet {
    pub async fn task(path: web::Path<String>) -> HttpResponse {
        let task_json = match serde_json::from_str(path.into_inner().as_str()) {
            Ok(task_json) => task_json,
            Err(e) => {
                log_error!("{}", e.to_string());
                return HttpResponse::NoContent().finish();
            }
        };

        HttpResponse::Ok().finish()
    }
}
