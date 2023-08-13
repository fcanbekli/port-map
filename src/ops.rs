use std::net::{SocketAddr, TcpStream};
use std::thread;
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
        const NUM_THREADS: usize = 100;
        let range_size = 65535 / NUM_THREADS;

        let handles: Vec<_> = (0..NUM_THREADS)
            .map(|thread_id| {
                let start = thread_id * range_size + 1;
                let end = if thread_id == NUM_THREADS - 1 {
                    65535
                } else {
                    (thread_id + 1) * range_size
                };

                let ip = self.ip.clone();

                thread::spawn(move || {
                    for i in start as u16..=end as u16 {
                        if scan_port(&ip, i) {
                            println!("Port {} active", i);
                        }
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().expect("Thread panicked");
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
