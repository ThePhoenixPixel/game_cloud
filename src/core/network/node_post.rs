use crate::core::service::Service;
use crate::log_error;
use crate::utils::logger::Logger;

use actix_web::{HttpResponse, web};


pub struct NodePost;

impl NodePost {
    pub async fn send_online_mode(path: web::Path<String>) -> HttpResponse {
        let _service_name = match serde_json::from_str(path.into_inner().as_str()) {
            Ok(service_name) => service_name,
            Err(e) => {
                log_error!("{}", e.to_string());
                return HttpResponse::NoContent().finish();
            }
        };
        let services = Service::get_all_service();
        for service in services {
            if service.get_name().starts_with("Proxy") {}
        }

        HttpResponse::Ok().finish()
    }
}