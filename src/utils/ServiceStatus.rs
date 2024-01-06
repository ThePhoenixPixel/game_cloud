

pub enum ServiceStatus {
    Start,
    Stop,
    Prepare,
}

impl ServiceStatus {

    pub fn isStart(&self) -> bool {
        match self {
            ServiceStatus::Start => true,
            _ => false,
        }
    }

    pub fn isStop(&self) -> bool {
        match self {
            ServiceStatus::Stop => true,
            _ => false,
        }
    }

    pub fn isPrepare(&self) -> bool { 
        match self { 
            ServiceStatus::Prepare => true,
            _ => false,
        }
    }
}