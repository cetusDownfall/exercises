pub struct Map {
    curr: bool,
    width: u32,
    height: u32,
    conts: [u8],
}
impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        if width%2 | height%2 == 0 {
            Map { curr: false, width, height, conts: [0; }
        } else {
            Map { curr: false, 16, 16, }
        }
    }
    pub fn place(
