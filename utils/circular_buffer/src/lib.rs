/// A circular buffer implementation on i32, has three parts: a boxed slice containing its
/// contents, a curr element index number, and a total size number.
pub struct CircularBuffer {
    conts: Box<[i32]>,
    w: usize,
    size: usize,
}

impl CircularBuffer {
    pub fn new(mut size: usize) -> CircularBuffer {
        if size==0{size = 1;}
        let conts=vec![0;size].into_boxed_slice();
        let w=size;
        CircularBuffer{conts,w,size}
    }
    pub fn push(&mut self, v: i32){self.w=(self.w+1)%self.size;if let Some(n)=self.conts.get_mut(self.w){*n=v;}}
    pub fn get_top(&self) -> Option<i32>{if let Some(&n)=self.conts.get(self.w){Some(n)}else{None}}
    pub fn get(&self, ind: usize) -> Option<i32>{if let Some(&n)=self.conts.get(ind){Some(n)}else{None}}
    pub fn get_conts(&self) -> Vec<i32>{(0..self.size).filter_map(|n| self.get(n)).collect::<Vec<i32>>()}
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_get_all() {
        use CircularBuffer;
        let mut c_buff = CircularBuffer::new(5);
        (0..5).for_each(|i| c_buff.push(i));
        assert_eq!(10, c_buff.get_conts().into_iter().sum());
        (5..10).for_each(|i| c_buff.push(i));
        assert_eq!(35, c_buff.get_conts().into_iter().sum());
    }
}
