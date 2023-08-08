use std::net::TcpStream;

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
            match TcpStream::connect(format!("{}:{}", self.ip, i)) {
                Ok(mut stream) => {
                    println!("Port Active {}", i);
                }
                Err(e) => {
                    println!("Error {}", i);
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
