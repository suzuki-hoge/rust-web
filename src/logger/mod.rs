use std::fmt::Display;

pub mod stdout;

pub trait Logger {
    fn info<S: Display>(&self, message: S);
}
