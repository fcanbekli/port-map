use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

pub fn scan_port(addr: &str, port: u16) -> bool {
    let server_addr = format!("{}:{}", addr, port).parse::<SocketAddr>();

    match server_addr {
        Ok(parsed_addr) => {
            match TcpStream::connect_timeout(&parsed_addr, Duration::from_millis(1)) {
                Ok(_) => true,
                Err(_) => false,
            }
        }
        Err(_) => {
            false // or handle the parsing error if needed
        }
    }
}

