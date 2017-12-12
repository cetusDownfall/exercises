fn main() {
    let mut i: u8 = 0;
    while i < 100 {
        if i%3 == 0 || i%5 == 0 {
            if i%3 == 0 {
                print!("Fizz");
            }
            if i%5 == 0 {
                print!("Buzz");
            }
        }else{
            print!("{}",i);
        }
	println!();
	i = i + 1;
    }
}
