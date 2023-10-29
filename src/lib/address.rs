pub struct Address {
    ip: String,
    port: u32,
}

impl Address {
    pub fn new() -> Address {
        return Address {
            ip: "127.0.0.1".to_string(),
            port: 60001,
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
}
