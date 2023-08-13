use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use crate::scanner::scan_port;

pub trait Op {
    fn execute(&self);
}

//UNKNOWN OP
pub struct UnknownOp {}

impl Op for UnknownOp {
    fn execute(&self) {
        println!("Unknown Operation");
    }
}
//UNKNOWN OP

//HELP OP
pub struct HelpOp {}

impl Op for HelpOp {
    fn execute(&self) {
        println!("Help Operation");
    }
}
//HELP OP

//SIMPLE FULL SCAN
pub struct FullScanOp {
    pub ip: String
}

impl Op for FullScanOp {
    fn execute(&self) {
            for i in 1..=65535 {
            if scan_port(&self.ip, i) {
                println!("Port {} active", i)
            }
        }
    }
}
//SIMPLE FULL SCAN

//PORT INTERVAL SCAN
pub struct PortsScanOp {
    pub ip: String,
    pub ports : Vec<u16>
}

impl Op for PortsScanOp {
    fn execute(&self) {
        for &port in &self.ports {
            if scan_port(&self.ip, port) {
                println!("Port {} active", port)
            }
        }
    }
}
//PORT INTERVAL SCAN

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown_op()
    {
    }
}
