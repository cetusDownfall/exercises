use std::time;
use std::mem;
fn seeded_byte(seed: u64) -> (u8, u64) {
    const RULE_NUM: u64 = 30;
    let size_in: u64 = 64; //std::mem::size_of_val(seed) as u8;
    let size_out: u64 = 8; //std::mem::size_of::<O>() as u8;
    let mut this_row: u64 = seed as u64;
    let mut next_row: u64 = 0;
    let mut out: u8 = 0;
    for row in 0..size_out {
        out |= ((this_row & 1u64)<<row) as u8;
        for column in 0..size_in {
            next_row |= (1 & (RULE_NUM>>(7 & this_row.rotate_left(1).rotate_right(column as u32))))<<column;
        }
        this_row = next_row;
        next_row = 0;
    }
    (out, this_row)
}
fn time_seeded_byte() -> (u8, u64) {
    seeded_byte(
        match time::SystemTime::now().duration_since(time::UNIX_EPOCH) { 
            Ok(n)   =>  ((n.as_secs() as u32) ^ n.subsec_nanos()) as u64,
            Err(_)  =>  42,
        })
}
pub struct WolfGen(u8,u64);//,i32);
impl WolfGen {
    pub fn new() -> WolfGen {
        let (val, seed) = time_seeded_byte();
        WolfGen(val, seed)//, 0)
    }
    pub fn next_bool(&mut self) -> bool { self.next_u8()%2==0 }
    pub fn next_u8(&mut self) -> u8 {
        //self.2 = (self.2+1)%309;
        //let mut val = 0;
        //let mut seed = 0;
        /*if self.2 == 0 {
            let (a, b) = time_seeded_byte();
            val = a;
            seed = b;
        } else {*/
            let (val, seed) /*a, b)*/ = seeded_byte(self.1);
            //val = a;
            //seed = b;
        //}
        self.0 = val;
        self.1 = seed;
        self.0
    }
    pub fn next_i8(&mut self) -> i8 { unsafe { mem::transmute::<u8, i8>(self.next_u8()) } }
    pub fn next_u16(&mut self) -> u16 {
        let v = [self.next_u8(); 2];
        unsafe {
           mem::transmute::<[u8; 2], u16>(v)
        }
    }
    pub fn next_i16(&mut self) -> i16 { unsafe { mem::transmute::<u16, i16>(self.next_u16()) } }
    pub fn next_u32(&mut self) -> u32 {
        let v = [self.next_u8(); 4];
        unsafe {
            mem::transmute::<[u8; 4], u32>(v)
        }
    }
    pub fn next_i32(&mut self) -> i32 { unsafe { mem::transmute::<u32, i32>(self.next_u32()) } }
    pub fn next_char(&mut self) -> char { unsafe { mem::transmute::<u32, char>(self.next_u32()) } }
    pub fn next_u64(&mut self) -> u64 {
        let v = [self.next_u8(); 8];
        unsafe {
            mem::transmute::<[u8; 8], u64>(v)
        }
    }
    pub fn next_i64(&mut self) -> i64 { unsafe { mem::transmute::<u64, i64>(self.next_u64()) } }
}
