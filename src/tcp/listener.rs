use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;

use crate::controller::ControllerResult;
use crate::logger::Logger;
use crate::tcp::request::{parse_request, Request};
use crate::tcp::response::Response;
use crate::{database, thread, LOGGER};

pub fn run<F>(ip_addr: IpAddr, port: u16, route: F) -> std::io::Result<()>
where
    F: Fn(Arc<database::mysql::Pool>, &Request) -> Result<ControllerResult, String> + 'static + Send + Sync,
{
    let socket = TcpListener::bind(SocketAddr::new(ip_addr, port))?;

    LOGGER.info("socket start");

    let thread_pool = thread::pool::Pool::new(3);
    let connection_pool = Arc::new(database::mysql::Pool::new("localhost", "13306", "app", "secret", "sales", 2));
    let route = Arc::new(route);

    for stream in socket.incoming() {
        let stream = stream?;

        let ref_connection_pool = connection_pool.clone();
        let ref_route = route.clone();
        thread_pool.execute(|| {
            let _ = handle(stream, ref_connection_pool, ref_route);
        });
    }

    Ok(())
}

fn handle<F>(mut stream: TcpStream, pool: Arc<database::mysql::Pool>, route: Arc<F>) -> std::io::Result<()>
where
    F: Fn(Arc<database::mysql::Pool>, &Request) -> Result<ControllerResult, String> + 'static + Send + Sync,
{
    let mut buf = [0; 1024];
    let _ = stream.read(&mut buf)?;

    let request = parse_request(String::from_utf8_lossy(&buf[..]));
    LOGGER.info(&request);

    let response = match route(pool, &request) {
        Ok(ControllerResult { status_code, content }) => Response { status_code, content },
        Err(e) => Response { status_code: 500, content: e },
    };
    LOGGER.info(&response);

    let _ = stream.write(&response.into_bytes())?;
    stream.flush()?;

    Ok(())
}
