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

    if args.get(1) == Some(&String::from("-s")) {
        let ip_address = args[2].clone();
        let op = Box::new(ops::SimpleFullScanOp { ip: ip_address });
        op.execute();
        return;
    }
}
