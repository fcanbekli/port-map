mod ops;
mod scanner;

use std::env;
use figlet_rs::FIGfont;
use crate::ops::Op;

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Port Map");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    let mut op: Box<dyn ops::Op>; // Use Box to work with trait objects
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        op = Box::new(ops::UnknownOp {});
        op.execute();
        return;
    }

    if args.get(1) == Some(&String::from("-h")) {
        op = Box::new(ops::HelpOp {});
        op.execute();
        return;
    }
    let args: Vec<String> = std::env::args().collect();

    if args.get(2) == Some(&String::from("-scan")) {
        if args.get(3) == Some(&String::from("-port")) {
            let ip_address = args[1].clone();
            let mut ports: Vec<u16> = Vec::new();
            for i in 4..args.len() {

                if let Some(port_str) = args.get(i) {
                    match port_str.parse::<u16>() {
                        Ok(parsed_num) => {
                            ports.push(parsed_num);
                        }
                        Err(_) => {
                            println!("Failed to parse the string as u16");
                        }
                    }
                } else {
                    println!("Option is None");
                }
            }
            let op = Box::new(ops::PortsScanOp { ip: ip_address, ports });
            op.execute();
            return;
        }

        let ip_address = args[1].clone();
        let op = Box::new(ops::FullScanOp { ip: ip_address });
        op.execute();
    }
}
