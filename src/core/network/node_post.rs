use std::time::Duration;
use crate::core::service::Service;
use crate::{log_info, log_warning};
use crate::utils::logger::Logger;

use actix_web::{HttpResponse, web};
use serde::Deserialize;
use crate::utils::service_status::ServiceStatus;

#[derive(Deserialize)]
pub struct SetOnlineModeRequest {
    name: String,
}

pub struct NodePost;

impl NodePost {
    pub async fn set_online_mode(service_request: web::Json<SetOnlineModeRequest>) -> HttpResponse {
        let service_name = &service_request.name;
        // Get the service obj. from the name
        let mut service = match Service::get_from_name(service_name) {
            Some(service) => service,
            None => {
                log_warning!("Service nicht gefunden {}", service_name);
                return HttpResponse::NoContent().finish();
            }
        };

        service.set_status(&ServiceStatus::Start);

        match service.connect_to_proxy().await {
            Ok(_) => log_info!("Service:  {} connect to Proxy", service.get_name()),
            Err(e) => log_warning!("Service: {} NOT connect to Proxy \n     Error -> {}", service.get_name(), e),
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
        HttpResponse::Ok().finish()
    }
}