use std::thread;
fn main() {
    let h = thread::spawn(|| {
        print!("Hello");
    });
    h.join().unwrap();
    print!(", ");

    let w = thread::spawn(|| {
        println!("World!");
    });
    w.join().unwrap();
}
