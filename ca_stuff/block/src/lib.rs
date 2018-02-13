pub mod margolus_gas;
pub mod randxor {
    pub struct Rand(usize);
    impl Rand {
        pub fn new(seed: usize) -> Rand {
            Rand(seed)
        }
        pub fn next(&mut self) -> usize {
            self.0 = random(self.0);
            self.0
        }
    }
    fn random(seed: usize) -> usize {
        seed ^ seed<<23
        ^
        seed ^ seed>>19
        ^
        seed ^ seed<<17
        ^
        seed ^ seed>>13
        ^
        seed ^ seed<<11
        ^
        seed ^ seed>>7
        ^
        seed ^ seed<<5
        ^
        seed ^ seed>>3
    }
}
