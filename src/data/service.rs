use crate::data::task::Task;

pub struct Service{
    name: String,
    status: String,
    online_players: u32,
    max_players: u32,
    max_ram: u32,
    use_ram: u32,
    task: Task,
}

impl Service{

}