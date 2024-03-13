pub trait CommandManager {
    fn execute(args: Vec<&str>) -> Result<(), String>;
    fn tab_complete(args: Vec<&str>) -> Vec<String>;
}
