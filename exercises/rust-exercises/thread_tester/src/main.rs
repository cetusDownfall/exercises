fn main() {
    const TIMES: i32 = 1500;
    let mut gen = PrimeGen(2);
    struct PrimeGen(i32);
    impl PrimeGen {
        fn next_prime(&mut self) -> i32 {
            self.0 += 1;
            let mut n = 2;
            loop {if (n * n) < self.0 { n+=1; }else{break;}}
            self.0 = 'outer: loop {
                for i in n..self.0 {
                    if self.0%i==0 { self.0 += 1; continue 'outer; }
                }
                break self.0;
            };
            self.0 as i32
        }
    }
    let mut count = 0;
    loop {
        if count==TIMES {break;}
        print!("{} ", gen.next_prime());
        count+=1;
    }
}

