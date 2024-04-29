use chrono::Local;
use std::fmt::Display;

use crate::logger::Logger;

pub struct Stdout {}

impl Logger for Stdout {
    fn info<S: Display>(&self, message: S) {
        let now = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
        println!("{} [{}] {}", now, Self::get_thread_info(), message);
    }
}
