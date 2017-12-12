fn main() {
    const UPPER_LIMIT: i32 = 16384;
    let mut sand: Vec<i32> = Vec::new();
    for n in 1..UPPER_LIMIT + 1 {sand.push(n);}
    let mut ind = 1;
    let mut n = sand[ind];
    while n < (UPPER_LIMIT / n) {sand = sieve(n, sand);ind += 1;n = sand[ind];}
    println!("Done Sieving.");print!("Primes: ");
    for n in sand {print!("{}, ", n);}

    fn sieve(hole: i32, debris: Vec<i32>) -> Vec<i32> {
        let mut out: Vec<i32> = Vec::new();
        println!("Sieving for: {}", hole);print!("Sieved");
        for i in debris {if i%hole == 0 && hole < i && hole > 1 {print!(" {}", i);} else {out.push(i);}}
        println!(".");println!("-------");out}
}
