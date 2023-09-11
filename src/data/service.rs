use crate::config::Config;
use crate::data::task::Task;
use crate::lib::bx;
use crate::lib::bx::Bx;

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