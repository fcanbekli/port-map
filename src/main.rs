mod ops;

use std::env;
use figlet_rs::FIGfont;

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
}
