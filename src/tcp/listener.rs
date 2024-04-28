use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};

use crate::controller::ControllerResult;
use crate::tcp::request::{parse_request, Request};
use crate::tcp::response::Response;

pub fn run<F>(ip_addr: IpAddr, port: u16, route: F) -> Result<(), String>
where
    F: Fn(&Request) -> Result<ControllerResult, String>,
{
    let socket = TcpListener::bind(SocketAddr::new(ip_addr, port)).map_err(|e| e.to_string())?;

    println!("socket start");
    for stream in socket.incoming() {
        let stream = stream.map_err(|e| e.to_string())?;

        println!("accepted");
        handle(stream, &route)?;
    }

    Ok(())
}

fn handle<F>(mut stream: TcpStream, route: &F) -> Result<(), String>
where
    F: Fn(&Request) -> Result<ControllerResult, String>,
{
    let mut buf = [0; 1024];
    let _ = stream.read(&mut buf).map_err(|e| e.to_string())?;

    let request = parse_request(String::from_utf8_lossy(&buf[..]));
    println!("{}", &request);

    let response = match route(&request) {
        Ok(ControllerResult { status_code, content }) => Response { status_code, content },
        Err(e) => Response { status_code: 500, content: e },
    };
    println!("{}", &response);

    let _ = stream.write(&response.into_bytes()).map_err(|e| e.to_string())?;
    stream.flush().map_err(|e| e.to_string())?;

    Ok(())
}
