use serde::Serialize;

use crate::core::service::Service;
use address::Address;

#[derive(Serialize, Debug)]
pub struct RegisterServerData {
    name: String,
    address: Address,
    try_to_connect: bool,
}

#[derive(Serialize, Debug)]
pub struct RegisterServerRequest {
    register_server: RegisterServerData,
}

impl RegisterServerData {
    pub fn create_request(service: &Service, try_to_connect: &bool) -> RegisterServerRequest {
        let register_server = RegisterServerData {
            name: service.get_name(),
            address: service.get_server_address(),
            try_to_connect: try_to_connect.clone(),
        };

        RegisterServerRequest {
            register_server,
        }
    }
}