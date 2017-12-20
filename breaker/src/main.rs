extern crate util;
use util::lsxr;

fn main() {
    let mut gen = lsxr::XGen::new();
    let mut lines = vec![gen.next_u16(); 16];
    loop {
        let mut frame = String::new();
        for i in &lines {
            frame.push('\n');
            for c in (0..16).rev() {
                if i & &(1<<c) != 0 {
                    frame.push_str("\u{2588}\u{2588}");
                } else {
                    frame.push_str("  ");
                }
            }
        }
        print!("{}", frame);
        generate_next_frame(frame);
    }
    fn generate_next_frame(conts: Vec<u16>) {
        for i in &conts {
        }
    }
}
