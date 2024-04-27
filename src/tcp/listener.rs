use std::io::{Error, Read, Write};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};

use crate::tcp::request::parse_request;
use crate::tcp::response::Response;

pub fn run(ip_addr: IpAddr, port: u16) -> Result<(), Error> {
    let socket = TcpListener::bind(SocketAddr::new(ip_addr, port))?;

    println!("socket start");
    for stream in socket.incoming() {
        let stream = stream?;

        println!("accepted");
        handle(stream)?;
    }

    Ok(())
}

fn handle(mut stream: TcpStream) -> Result<(), Error> {
    let mut buf = [0; 1024];
    let _ = stream.read(&mut buf)?;

    let request = parse_request(String::from_utf8_lossy(&buf[..]));
    println!("{:?}", &request);

    let (status_code, content) = (200, String::from(r#"{"result": "ok"}"#));
    let response = Response { status_code, content };
    println!("{:?}", &response);

    let _ = stream.write(&response.into_bytes())?;
    stream.flush()?;

    Ok(())
}
