use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use crate::scanner::scan_port;

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
            if scan_port(&self.ip, i) {
                println!("Port {} active", i)
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
