use std::net::IpAddr;

use crate::controller::route;

mod controller;
mod database;
mod tcp;

fn main() -> Result<(), String> {
    tcp::listener::run(IpAddr::V4("0.0.0.0".parse().unwrap()), 9090, route)
}
