pub struct Point_64(u8);
pub struct BitArray_64(u64);
impl Point_64 {
    pub fn new() -> Point_64 {
        Point_64(0)
    }
    pub fn get_row(&self) -> u8 {
        (self.0>>4)&7
    }
    pub fn get_col(&self) -> u8 {
        self.0&7
    }
    pub fn set(&mut self, row: u8, col: u8) {
        self.0 = ((row & 7)<<4 | (col & 7))
    }
    pub fn set_row(&mut self, row: u8) {
        self.0 &= 7;
        self.0 |= (7 & row)<<4;
    }
    pub fn set_col(&mut self, col: u8) {
        self.0 &= 7<<4;
        self.0 |= col;
    }
}
impl BitArray_64 {
    pub fn new() -> BitArray_64 {
        BitArray_64(0)
    }
    pub fn get_point(&self, pt: Point) -> init
