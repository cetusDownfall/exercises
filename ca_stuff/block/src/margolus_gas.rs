use randxor::Rand;
pub struct Gas {
    conts: Vec<Vec<u8>>,
    step: ShiftState,
}
#[derive(Debug, PartialEq)]
enum ShiftState {
    Shift,
    Unshift,
}
impl Gas {
    pub fn new(size: usize, density: f32, framed: bool) -> Gas {
        let mut rows: Vec<Vec<u8>> = Vec::with_capacity(size);
        let mut random = Rand::new(size);
        for _ in 0..size {
            let mut col: Vec<u8> = Vec::with_capacity(size);
            for _ in 0..size {
                if Rand::next(&mut random)%((size as f32 * density) as usize) == 0 {
                    col.push(1)
                } else {
                    col.push(0);
                }
            }
            rows.push(col);
        }
        let mut out = Gas::from(rows);
        if framed { out.frame(); }
        out
    }
    pub fn from(vdq: Vec<Vec<u8>>) -> Gas {
        Gas {
            conts: vdq,
            step: ShiftState::Shift,
        }
    }
    pub fn hole(&mut self, size: usize) {
        let pos = self.conts.len()/2 + size/2;
        let neg = self.conts.len()/2 - size/2;
        self.conts.iter_mut().enumerate().filter(|&(i, _)| i>neg && i<pos).flat_map(|(_, row)| row.iter_mut().enumerate()).for_each(|(i, n)| if i>neg && i<pos { *n = 0 });
        self.frame();
    }
    pub fn block(&mut self, size: usize) {
        let pos = self.conts.len()/2 + size/2;
        let neg = self.conts.len()/2 - size/2;
        self.conts.iter_mut().enumerate().for_each(|(i, row)| row.iter_mut().enumerate().for_each(|(j, n)| if !(j<pos && j>neg && i<pos && i>neg) { *n = 0 }));
        self.frame();
    }
    pub fn generate_frames(&mut self, frames: usize) -> Vec<Vec<(f64, f64, u8)>> {
        (0..).take(frames).map(|_| self.pass_state()).collect::<Vec<Vec<(f64, f64, u8)>>>()
    }
    fn frame(&mut self) {
        let a = self.conts.pop().unwrap(); let b = self.conts.pop().unwrap();
        self.conts.iter_mut().for_each(|row| { row.pop(); row.pop(); row.push(255); row.push(255); });
        self.conts.push(b.into_iter().map(|_| 255).collect::<Vec<_>>()); self.conts.push(a.into_iter().map(|_| 255).collect::<Vec<_>>());
        self.shift();
    }
    fn shift(&mut self) {
        {
            let tmp = self.conts.pop().unwrap();
            self.conts.insert(0, tmp);
        }
        self.conts.iter_mut()
            .for_each(|row| {
            let tmp = row.pop().unwrap();
            row.insert(0, tmp);
        });
    }
    fn unshift(&mut self) {
        {
            let tmp_iter: Vec<_> = self.conts.drain(1..).collect();
            let tmp = self.conts.pop().unwrap();
            self.conts.extend(tmp_iter);
            self.conts.push(tmp);
        }
        self.conts.iter_mut()
            .for_each(|row| {
            let tmp_iter: Vec<_> = row.drain(1..).collect();
            let tmp = row.pop().unwrap();
            row.extend(tmp_iter);
            row.push(tmp);
        });
    }
    pub fn pass_state(&mut self) -> Vec<(f64, f64, u8)> {
        if let Some(states) = self.pass_gas() {
            states
        } else {
            self.pass_gas().unwrap()
        }
    }
    fn pass_gas(&mut self) -> Option<Vec<(f64, f64, u8)>> {
        if self.step == ShiftState::Unshift {
            self.step = ShiftState::Shift;
            self.unshift();
            let mut out = Vec::with_capacity((self.conts.len()>>1) * (self.conts.len()>>1));
            self.conts
                .chunks_mut(2)
                .map(|two_chunks| two_chunks.split_at_mut(1))
                .for_each(|(a, b)| { 
                    let mut iter = a[0].iter_mut().by_ref().zip(b[0].iter_mut().by_ref()).collect::<Vec<_>>();
                    while let (Some(a), Some(b)) = (iter.pop(), iter.pop()) {
                        let preconditions = [*a.0, *a.1, *b.0, *b.1];
                        let count = preconditions.iter().filter(|n| **n != 0).count();
                        out.push(count as u8);
                        /* Critters rule. LAGGY.
                        let count = preconditions.iter().filter(|n| **n != 0).count();
                        if count != 2 {
                            *a.0 ^= 1;
                            *a.1 ^= 1;
                            *b.0 ^= 1;
                            *b.1 ^= 1;
                        }
                        */
                        
                        /* Tron rule.
                        let count = preconditions.iter().filter(|n| **n != 0).count();
                        if count == 4 {
                            *a.0 = 0; *a.1 = 0; *b.0 = 0; *b.1 = 0;
                        } else if count == 0 {
                            *a.0 = 1; *a.1 = 1; *b.0 = 1; *b.1 = 1;
                        }
                        */
                        //HPP Gas rule.
                        let count = preconditions.iter().filter(|n| **n == 255).count();
                        if count == 0 {
                            *a.0 = preconditions[3];
                            *a.1 = preconditions[2];
                            *b.0 = preconditions[1];
                            *b.1 = preconditions[0];
                        }
                        // BBM rule.
                        /*
                        let count = preconditions.iter().filter(|n| **n != 0).count();
                        if count == 2 {
                            if !(*a.0 + *a.1 > 1 || *b.0 + *b.1 > 1) {
                                *b.0 = preconditions[0];
                                *b.1 = preconditions[1]; 
                                *a.0 = preconditions[2]; 
                                *a.1 = preconditions[3];
                            }
                        } else if count == 1 {
                            *a.0 = preconditions[3];
                            *a.1 = preconditions[2];
                            *b.0 = preconditions[1];
                            *b.1 = preconditions[0];
                        }
                        */
                    }
                });
                out.reverse();
                Some(out.into_iter().enumerate().map(|(i, n)| ((i % (self.conts.len()>>1)) as f64, (i / (self.conts.len()>>1)) as f64, n)).collect::<Vec<(f64, f64, u8)>>())
        } else {
            self.step = ShiftState::Unshift;
            self.shift();
            self.conts
                .chunks_mut(2)
                .map(|two_chunks| two_chunks.split_at_mut(1))
                .for_each(|(a, b)| { 
                    let mut iter = a[0].iter_mut().by_ref().zip(b[0].iter_mut().by_ref()).collect::<Vec<_>>();
                    while let (Some(a), Some(b)) = (iter.pop(), iter.pop()) {
                        let preconditions = [*a.0, *a.1, *b.0, *b.1];
                        let count = preconditions.iter().filter(|n| **n != 0).count();
                        /* Critters rule. LAGGY.
                        let count = preconditions.iter().filter(|n| **n != 0).count();
                        if count != 2 {
                            *a.0 ^= 1;
                            *a.1 ^= 1;
                            *b.0 ^= 1;
                            *b.1 ^= 1;
                        }
                        */
                        
                        /* Tron rule.
                        let count = preconditions.iter().filter(|n| **n != 0).count();
                        if count == 4 {
                            *a.0 = 0; *a.1 = 0; *b.0 = 0; *b.1 = 0;
                        } else if count == 0 {
                            *a.0 = 1; *a.1 = 1; *b.0 = 1; *b.1 = 1;
                        }
                        */
                        //HPP Gas rule.
                        let count = preconditions.iter().filter(|n| **n == 255).count();
                        if count == 0 {
                            *a.0 = preconditions[3];
                            *a.1 = preconditions[2];
                            *b.0 = preconditions[1];
                            *b.1 = preconditions[0];
                        }
                        // BBM rule.
                        /*
                        let count = preconditions.iter().filter(|n| **n != 0).count();
                        if count == 2 {
                            if !(*a.0 + *a.1 > 1 || *b.0 + *b.1 > 1) {
                                *b.0 = preconditions[0];
                                *b.1 = preconditions[1]; 
                                *a.0 = preconditions[2]; 
                                *a.1 = preconditions[3];
                            }
                        } else if count == 1 {
                            *a.0 = preconditions[3];
                            *a.1 = preconditions[2];
                            *b.0 = preconditions[1];
                            *b.1 = preconditions[0];
                        }
                        */
                    }
                });
            None
        }
    }
}
