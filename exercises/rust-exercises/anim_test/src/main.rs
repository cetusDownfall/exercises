use std::thread;
use std::time::Duration;

fn main() {
    const TERM_SIZE: usize = 57;
    const ROWS: usize = 16;
    const COLS: usize = 32;
    const ON: char = '\u{25A0}';
    const OFF: char = ' ';
    let mut n = 0;
    while n < 100 {
        let frame = vec![vec![if n%2 == 0 { ON } else { OFF }; COLS]; ROWS];
        for _ in 0..(TERM_SIZE-ROWS) { println!(); }
        for r in frame.iter() {
            for c in r.iter() { print!("{}", c); }
            println!();}
        thread::sleep(Duration::from_secs(2));
        n+=1;
    }
}
