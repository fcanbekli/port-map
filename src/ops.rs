use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

pub trait Op {
    fn execute(&self);
}

pub struct UnknownOp {}

impl Op for UnknownOp {
    fn execute(&self) {
        println!("Unknown Operation");
    }
}

pub struct HelpOp {}

impl Op for HelpOp {
    fn execute(&self) {
        println!("Help Operation");
    }
}

pub struct SimpleFullScanOp {
    pub ip: String
}

impl Op for SimpleFullScanOp {
    fn execute(&self) {
        for i in 1..=5000 {
            let server_addr = format!("{}:{}", self.ip, i).parse::<SocketAddr>();

            match server_addr {
                Ok(addr) => {
                    match TcpStream::connect_timeout(&addr, Duration::from_millis(1)) {
                        Ok(mut stream) => {
                            println!("Port Active {}", i);
                        }
                        Err(_) => {}
                    }
                }
                Err(e) => {
                    println!("Error parsing address: {}", e);
                    return;
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown_op()
    {
    }
}
