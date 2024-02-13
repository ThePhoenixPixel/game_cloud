pub trait CommandManager {
    fn execute(args: Vec<&str>);
    fn tab_complete(args: Vec<&str>) -> Vec<String>;
}
