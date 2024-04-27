use std::io::{Error, Read, Write};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};

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

    let request: String = String::from_utf8_lossy(&buf[..]).to_string();
    println!("{}", request);

    let response = "HTTP/1.1 200 OK\r\n";
    let _ = stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}
