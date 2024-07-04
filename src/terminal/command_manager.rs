pub trait CommandManager {
    async fn execute(args: Vec<&str>) -> Result<(), String>;
    fn tab_complete(args: Vec<&str>) -> Vec<String>;
}
