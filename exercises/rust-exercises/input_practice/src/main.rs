use std::io;

fn main() {
    let mut string_in = String::new();
    if let Ok(n) = io::stdin().read_line(&mut string_in) {
        println!("{} things input", n);
        println!("{}", string_in);
    }
    for w in string_in.split_whitespace() {
        println!("{}", w);
    }
}
