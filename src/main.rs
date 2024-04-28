use std::io::Error;
use std::net::IpAddr;

use crate::controller::route;

mod controller;
mod tcp;

fn main() -> Result<(), Error> {
    tcp::listener::run(IpAddr::V4("0.0.0.0".parse().unwrap()), 9090, route)
}
