use std::thread;
use std::sync::{Arc, Mutex, mpsc};

fn main() {
    let (tx, rx) = mpsc::channel();
    let hw = Arc::new(Mutex::new(vec!['!', 'd', 'l', 'r', 'o', 'W', ' ', ',', 'o', 'l', 'l', 'e', 'H']));
    for _ in 0..13 {
        let tx = tx.clone();
        let hw = hw.clone();
        thread::spawn(move || {
            let mut c = hw.lock().unwrap();
            //c.reverse();
            tx.send(c.pop().unwrap()).unwrap();
        });
    }
    for _ in 0..13 {
        print!("{}",  rx.recv().unwrap());
    }
}
