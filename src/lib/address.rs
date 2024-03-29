use serde::{Deserialize, Serialize};
use std::net::TcpListener;

#[derive(Serialize, Deserialize, Clone)]
pub struct Address {
    ip: String,
    port: u32,
}

impl Address {
    pub fn new(ip: &String, port: &u32) -> Address {
        return Address {
            ip: ip.clone(),
            port: port.clone(),
        };
    }

    pub fn get_ip(&self) -> String {
        self.ip.clone()
    }

    pub fn set_ip(&mut self, ip: &String) {
        self.ip = ip.clone();
    }

    pub fn get_port(&self) -> u32 {
        self.port.clone()
    }

    pub fn set_port(&mut self, port: u32) {
        self.port = port.clone();
    }

    pub fn find_next_port(address: &mut Address) -> u32 {
        let mut port = address.get_port();
        let max_port: u32 = 65535;
        while port <= max_port {
            address.set_port(port);
            if Address::is_port_available(address) {
                return port; // Verwende 'return' hier, um den gefundenen Port zurückzugeben
            }
            port += 1;
        }
        //keine ahnung was ich machen soll
        panic!("Error es ist kein freier Port gefunden worden");
    }

    pub fn is_port_available(address: &Address) -> bool {
        let socket_addr = format!("{}:{}", address.get_ip(), address.get_port());
        if let Ok(listener) = TcpListener::bind(&socket_addr) {
            // Port ist verfügbar
            drop(listener);
            true
        } else {
            // Port ist bereits in Verwendung
            false
        }
    }
    pub fn to_string(&self) -> String {
        format!("{}:{}", self.get_ip(), self.get_port()).to_string()
    }
}
