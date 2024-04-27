use std::io::Error;
use std::net::IpAddr;

mod tcp;

fn main() -> Result<(), Error> {
    tcp::listener::run(IpAddr::V4("0.0.0.0".parse().unwrap()), 9090)
}
