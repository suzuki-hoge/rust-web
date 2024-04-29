use chrono::Local;
use std::fmt::Display;
use std::thread;

use crate::logger::Logger;

pub struct Stdout {}

impl Logger for Stdout {
    fn info<S: Display>(&self, message: S) {
        let now = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
        let thread = thread::current();
        println!("{} [thread {}] {}", now, thread.id().as_u64(), message);
    }
}
