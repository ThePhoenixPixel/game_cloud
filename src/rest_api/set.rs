use actix_web::{HttpResponse, web};
use crate::logger::Logger;


pub struct ApiSet;

impl ApiSet {

    pub async fn task(path: web::Path<String>) -> HttpResponse {
        let task_json = match serde_json::from_str(path.into_inner().as_str()) {
            Ok(task_json) => task_json,
            Err(e) => {
                Logger::error(e.to_string().as_str());
                return HttpResponse::NoContent().finish();
            }
        };


        HttpResponse::Ok().finish()
    }

}