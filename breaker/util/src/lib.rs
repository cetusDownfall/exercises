pub fn wolfram_30_prng(seed: &i32, length: &i32) -> Vec<u8> {
    const RULE_NUM: i32 = 30;
    let seed_length = 32;
    let total_rows = 8* *length;
    let mut out: Vec<u8> = Vec::new();
    let mut next_row = 0;
    let mut this_row = *seed;
    {
        let mut row = 0;
        let mut next_byte = 0u8;
        while row < total_rows {
            for i in 0..seed_length {
                let this_state = 
                    if i < 31 && i > 0 { 
                        this_row>>(i-1) & 7 
                    } else { 
                        (7 &
                            ((this_row & 1) << 
                            if i == 0 { 1 } 
                            else { 2 } 
                        | 
                            (this_row & (1<<31)) >> 
                            if i == 0 { 31 } 
                            else { 30 } 
                        |  
                            if i == 0 { this_row & 4 }
                            else { (this_row & 30)>>30 }))
                    };
                next_row |= ((RULE_NUM>>this_state) & 1)<<i;
            }
            next_byte |= ((this_row & 1)<<(row%8)) as u8;
            for i in (0..seed_length).rev() {
                print!("{} ", this_row>>i & 1);
            }
            println!("");
            this_row = next_row;
            next_row = 0;
            if row%8 == 7 {
                out.push(next_byte);
                next_byte = 0;
            }
            row += 1;
        }
    }
    out
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
