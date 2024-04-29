use std::fmt::Display;
use std::thread;

pub mod stdout;

pub trait Logger {
    fn info<S: Display>(&self, message: S);

    fn get_thread_info() -> String {
        let thread = thread::current();
        match thread.id().as_u64().get() {
            1 => "thread main".to_string(),
            n => format!("thread {}", n - 1),
        }
    }
}
