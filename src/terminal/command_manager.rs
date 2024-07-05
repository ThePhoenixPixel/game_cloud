pub trait CommandManager {
    fn execute(args: Vec<&str>) -> impl std::future::Future<Output=Result<(), String>> + Send;
    fn tab_complete(args: Vec<&str>) -> Vec<String>;
}
