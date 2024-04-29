#![feature(thread_id_value)]

use std::net::IpAddr;

use crate::controller::route;
use crate::logger::stdout::Stdout;

mod cache;
mod controller;
mod database;
mod logger;
mod tcp;
mod thread;

pub static LOGGER: Stdout = Stdout {};

fn main() -> Result<(), String> {
    tcp::listener::run(IpAddr::V4("0.0.0.0".parse().unwrap()), 9090, route).map_err(|e| e.to_string())
}
