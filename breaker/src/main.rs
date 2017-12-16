extern crate util;

fn main() {
    let f: Vec<u8> = util::wolfram_30_prng(&48, &8192);
    for i in f {
        println!("{}", i);
    }
}
