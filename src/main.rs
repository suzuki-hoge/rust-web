#![feature(thread_id_value)]

use std::net::IpAddr;

use crate::controller::route;
use crate::logger::stdout::Stdout;

use structopt::StructOpt;

mod cache;
mod controller;
mod database;
mod logger;
mod tcp;
mod thread;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long)]
    pub thread: u8,
    #[structopt(long)]
    connection: u8,
}

pub static LOGGER: Stdout = Stdout {};

fn main() -> Result<(), String> {
    let opt: Opt = Opt::from_args();

    tcp::listener::run(IpAddr::V4("0.0.0.0".parse().unwrap()), 9090, opt.thread, opt.connection, route)
        .map_err(|e| e.to_string())
}
