extern crate vectils;
use vectils::Vector2d;
fn main() {
    let v1 = [0., 0., 5., 5.];
    let v2 = [0., 5., 5., 0.];
    let out = v1.cross(&v2);
    if let Some(n) = out {
        println!("{}, {}", n[0], n[1]);
    }
}
