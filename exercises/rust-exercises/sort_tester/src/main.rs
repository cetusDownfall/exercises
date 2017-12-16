
extern crate rand;
extern crate quick_sort;

use rand::Rng;

fn main() {
    use quick_sort::int_sorter;
    const LIMIT: usize = 512;
    let mut seq = [0; LIMIT];
    rand::thread_rng().fill_bytes(&mut seq);
    for i in 0..seq.len()-1 {
        print!("{} ", seq[i]);
    }
    quick_sort::int_sorter(seq);
    for i in 0..seq.len()-1 {
        print!("{} ", seq[i]);
    }
}
