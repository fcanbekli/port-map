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
            false
        }
    }
}

fn parse_ip(addr: &str, port: u16) -> Result<SocketAddr, AddrParseError> {
    return format!("{}:{}", addr, port).parse::<SocketAddr>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ip() {
        match parse_ip("127.0.0.1", 15) {
            Ok(parsed_addr) => {
                assert_eq!(parsed_addr.to_string(), "127.0.0.1:15");
            }
            Err(_) => {
                assert!(false, "Failed to parse IP address");
            }
        }
    }
}