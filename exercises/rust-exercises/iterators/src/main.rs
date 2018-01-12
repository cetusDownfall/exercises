use std::iter::{empty, once};
use std::time::{SystemTime, UNIX_EPOCH};
fn doors() {
    vec![false; 100].iter_mut().enumerate().for_each(
        |(d, ds)| print!("{}", match (1..101).into_iter().filter(|p| (d+1)%p==0).map(|_| {*ds=!*ds;*ds}).last().unwrap(){ true => "/", false => "_",}));
}
fn primes(limit: usize) -> Box<Iterator<Item=usize>> {
    if limit <3 {return if limit * limit==limit{Box::new(empty())}else{Box::new(once(2))}}
    let mut is_prime = vec![true; limit+1];
    is_prime[0]=false;
    if limit >= 1{is_prime[1]=false}
    let sqrtlmt = (limit as f64).sqrt() as usize + 1;
    for num in 2..sqrtlmt {if is_prime[num] {let mut multiple = num * num;while multiple <= limit {is_prime[multiple] = false;multiple += num;}}} 
    Box::new(is_prime.into_iter().enumerate().filter_map(|(p, is_prm)| if is_prm { Some(p) } else { None }))
}
struct Rng(u64);
/* Later.
enum Number {
    Byte(bool),
    Bytee(bool),
    Chomp(bool),
    Long(bool),
}
*/
impl Iterator for Rng {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        self.0 ^= self.0<<19;
        self.0 ^= self.0>>23;
        self.0 ^= self.0<<5;
        Some(self.0)
    }
}
impl Rng {
    fn new() -> Rng {Rng(if let Ok(n) = SystemTime::now().duration_since(UNIX_EPOCH) { n.as_secs() } else { 1 })}
    fn get_u8(&mut self) -> u8 { self.next().unwrap() as u8 }
    fn get_char(&mut self) -> char { self.skip_while(|n| (*n as u8 as char).is_whitespace() || (*n as u8 as char).is_control()).next().unwrap() as u8 as char }
}
fn main() {
    //doors();
    //println!("{}", primes(1000000).last().unwrap());
    //println!("{}, {}", (0..).take_while(|x| x * x < 3029).last().unwrap(), 3029f64.sqrt());
    (0..!0u8).map(|n|n as char).filter(|n|!n.is_control()&&!n.is_whitespace()).enumerate().for_each(|(i, c)|print!("{}{}",c,if(i+1)%16==0{"\n"}else{""}));
    //let mut jeez = Rng::new();
    //(0..100).for_each(|_| { (0..60).for_each(|_|  print!("{}", jeez.get_char())); println!(); } );
}
