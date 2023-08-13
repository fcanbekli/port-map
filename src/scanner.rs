use std::net::{AddrParseError, SocketAddr, TcpStream};
use std::time::Duration;

pub fn scan_port(addr: &str, port: u16) -> bool {
    match parse_ip(addr, port) {
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

fn parse_ip(addr: &str, port: u16) -> Result<SocketAddr, AddrParseError> {
    return format!("{}:{}", addr, port).parse::<SocketAddr>();
}