use figlet_rs::FIGfont;

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Port Map");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
}
