fn time_seeded_u64() -> u64 {
    use std::time;
    match time::SystemTime::now().duration_since(time::UNIX_EPOCH) {
        Ok(n)   =>  n.as_secs(),
        Err(_)  =>  42u64,
    }
}
fn shifter(val: u64) -> u64 {
    val ^ val<<3
    ^
    val ^ val>>5
    ^
    val ^ val<<7
    ^
    val ^ val>>11
    ^
    val ^ val<<13
    ^
    val ^ val>>17
    ^
    val ^ val<<19
    ^
    val ^ val>>23
    ^
    val ^ val<<29
    ^
    val ^ val>>31
}
pub struct XGen(u64);
impl XGen {
    pub fn new() -> XGen {
        XGen(time_seeded_u64())
    }
    pub fn next_bool(&mut self) -> bool {
        self.next()%2==0
    }
    pub fn next(&mut self) -> u64 {
        self.0 = shifter(self.0);
        self.0
    }
    pub fn next_u8(&mut self) -> u8 {
        ((255<<30 & self.next())>>30) as u8
    }
    pub fn next_u16(&mut self) -> u16 {
        ((!0<<30 as u16 & self.next())>>30) as u16
    }
    pub fn next_u32(&mut self) -> u32 {
        ((!0<<30 as u32 & self.next())>>30) as u32
    }
}
