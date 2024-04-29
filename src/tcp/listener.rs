use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use crate::controller::ControllerResult;
use crate::logger::Logger;
use crate::tcp::request::{parse_request, Request};
use crate::tcp::response::Response;
use crate::LOGGER;

pub fn run<F>(ip_addr: IpAddr, port: u16, route: F) -> std::io::Result<()>
where
    F: Fn(&Request) -> Result<ControllerResult, String> + 'static + Send + Sync,
{
    let route = Arc::new(route);

    let socket = TcpListener::bind(SocketAddr::new(ip_addr, port))?;

    LOGGER.info("socket start");

    for stream in socket.incoming() {
        let stream = stream?;

        let ref_route = route.clone();
        thread::spawn(|| {
            let _ = handle(stream, ref_route);
        });
    }

    Ok(())
}

fn handle<F>(mut stream: TcpStream, route: Arc<F>) -> std::io::Result<()>
where
    F: Fn(&Request) -> Result<ControllerResult, String> + 'static + Send + Sync,
{
    let mut buf = [0; 1024];
    let _ = stream.read(&mut buf)?;

    let request = parse_request(String::from_utf8_lossy(&buf[..]));
    LOGGER.info(&request);

    let response = match route(&request) {
        Ok(ControllerResult { status_code, content }) => Response { status_code, content },
        Err(e) => Response { status_code: 500, content: e },
    };
    LOGGER.info(&response);

    let _ = stream.write(&response.into_bytes())?;
    stream.flush()?;

    Ok(())
}
