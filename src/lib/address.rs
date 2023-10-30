use std::net::TcpListener;

pub struct Address {
    ip: String,
    port: u32,
}

impl Address {
    pub fn new(ip: &String, port: &u32) -> Address {
        return Address {
            ip: ip.clone(),
            port: port.clone(),
        }
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


    pub fn find_next_port(ip: &String, start_port: u32) -> Option<u32> {
        let mut port = start_port;
        let max_port: u32 = 65535;
        while port <= max_port {
            if is_port_available(ip, port) {
                return Some(port); // Verwende 'return' hier, um den gefundenen Port zurückzugeben
            }
            port += 1;
        }
        println!("Error es ist kein freier Port gefunden worden");
        None
    }
}

fn is_port_available(host: &String, port: u32) -> bool {
    let socket_addr = format!("{}:{}", host, port);
    if let Ok(listener) = TcpListener::bind(&socket_addr) {
        // Port ist verfügbar
        drop(listener);
        true
    } else {
        // Port ist bereits in Verwendung
        false
    }
}
