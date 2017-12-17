pub fn seeded_byte(seed: u8) -> u8 {
    const RULE_NUM: u8 = 30;
    const SIZE: u8 = 8;
    let mut this_row = seed;
    let mut next_row = 0;
    let mut out = 0;
    for row in 0..SIZE {
        out |= (this_row & 1)<<row;
        for column in 0..SIZE {
            next_row |= (1 & (RULE_NUM>>(match column {
                    0   =>  (this_row<<1 & 6) | (this_row>>7 & 1),
                    7   =>  (this_row>>6 & 3) | (this_row<<2 & 1),
                    _   =>  7 & (this_row>>(column-1)),
                })))<<column;
        }
        this_row = next_row;
        next_row = 0;
    }
    out
}
pub fn random_byte() -> u8 {
    seeded_byte(
        match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) { 
            Ok(n)   =>  ((n.as_secs() as u32)^n.subsec_nanos()) as u8,
            Err(_)  =>  16,
        })
}
/*pub fn wolfram_30_prng(seed: i32, length: i32) -> Vec<u8> {
    const RULE_NUM: i32 = 30;
    const SEED_LENGTH: i32 = 32;
    let total_rows: usize = 8* length as usize;
    let mut out: Vec<u8> = Vec::new();
    let mut next_row = 0;
    let mut this_row = seed;
    let mut next_byte = 0;
    for row in 0..total_rows {
        for column in (0..SEED_LENGTH).rev() {
            let this_state = 
                if column < 31 && column > 0 { 
                    this_row>>(column-1) & 7 
                } else { 
                    (7 &
                        ((this_row & 1) << 
                        if column == 0 { 1 } 
                        else { 2 } 
                    | 
                        (this_row & (1<<31)) >> 
                        if column == 0 { 31 } 
                        else { 30 } 
                    |  
                        if column == 0 { this_row & 4 }
                        else { (this_row & 30)>>30 }))
                };
            print!("{} ", this_row>>column & 1);
            next_row |= ((RULE_NUM>>this_state) & 1)<<column;
        }
        println!("");
        next_byte |= ((this_row & 1)<<(row%8)) as u8;
        if row%8 == 7 { out.push(next_byte); next_byte = 0; }
        this_row = next_row;
        next_row = 0;
    }
    out
}*/
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
