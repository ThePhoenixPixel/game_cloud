use std::thread;

pub struct ThreadManager {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadManager {
    pub fn new() -> ThreadManager {
        ThreadManager { threads: Vec::new() }
    }

    pub fn spawn<F>(&mut self, func: F) -> &mut ThreadManager
        where
            F: FnOnce() + Send + 'static,
    {
        let handle = thread::spawn(func);
        self.threads.push(handle);
        self
    }

    pub fn shutdown_all(&mut self) {
        for thread in self.threads.drain(..) {
            thread.join().unwrap();
        }
    }
}
