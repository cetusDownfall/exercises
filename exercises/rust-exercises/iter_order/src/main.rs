fn main() {
    let mut v = Vec::new();
    for n in 0..100 {
        v.push(n);
    }
    println!("{:?}", v.pop().unwrap());
    for n in v.into_iter() {
        println!("{:?}", n);
    }
}
