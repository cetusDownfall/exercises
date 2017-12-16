extern crate rand;
extern crate quick_sort;

fn main() {
    use rand::Rng;
    const LIMIT: usize = 512;
    let mut seq = [0u8; LIMIT];
    rand::thread_rng().fill_bytes(&mut seq[..]);
    for i in 0..seq.len()-1 {
        print!("{} ", seq[i]);
    }
    println!("\n------");
    quick_sort::int_sorter::pivot(&mut seq[..]);
    for i in 0..seq.len()-1 {
        print!("{} ", seq[i]);
    }
}
