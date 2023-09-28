use crate::data::task::Task;

enum Status {
    Start,
    Prepare,
    Stop,
}

pub struct Service{
    name: String,
    status: Status,
    online_players: u32,
    max_players: u32,
    max_ram: u32,
    use_ram: u32,
    task: Task,
}

impl Service{

}