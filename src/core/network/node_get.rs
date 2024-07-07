use std::time::Duration;
use crate::core::service::Service;
use crate::lib::address::Address;

use actix_web::HttpResponse;
use serde::{Serialize};


pub struct NodeGet;

#[derive(Serialize, Debug)]
pub(crate) struct ServiceInfo {
    pub(crate) name: String,
    pub(crate) server_address: Address,
}

impl NodeGet {
    pub async fn online_service() -> HttpResponse {
        let online_service = Service::get_online_backend_server();

        let mut services_info: Vec<ServiceInfo> = Vec::new();
        for service in online_service {
            let service_name = service.get_name();
            let server_address = service.get_server_address();
            services_info.push(ServiceInfo {
                name: service_name,
                server_address,
            });
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
        return match serde_json::to_string(&services_info) {
            Ok(data) => HttpResponse::Ok().json(data),
            Err(_) => return HttpResponse::NoContent().finish(),
        };
    }
}
