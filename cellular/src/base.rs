pub struct CAGrid { //to be read as Cellular Automata Grid.
    size: usize, //generic num type and variable r/c values later maybe?
    contents: Vec<u8>,//VecDeque<VecDeque<<u8>>>, //u8 is standing in for a state. generic state later. Also try using just a one deep vdq later, maybe faster?
}
impl CAGrid {
    pub fn new(size: usize) -> Self {
        let mut contents = VecDeque::with_capacity(size); //size for this is number of blocks, not cells.
        for _ in 0..size {
            let mut row = VecDeque::with_capacity(size);
            for _ in 0..size {
                row.push_back(RefCell::new(DEFAULT));
            }
            contents.push_back(row);
        }
        CAGrid {
            size,
            contents,
        }
    }
    pub fn with_starting(mut states: Vec<u8>) -> Self { 
        let mut contents = VecDeque::new();
        let mut first_row = VecDeque::new();
        while (first_row.len() * first_row.len()) < states.len() {
            if let Some(state) = states.pop() {
                first_row.push_back(RefCell::new(state));
            } else {
                break;
            }
        }
        let size = first_row.len();
        while contents.len() < size {
            let mut row = VecDeque::with_capacity(size);
            while row.len() < size {
                if let Some(state) = states.pop() {
                    row.push_back(RefCell::new(state));
                } else {
                    row.push_back(RefCell::new(DEFAULT));
                }
            }
            contents.push_back(row);
        }
        contents.push_front(first_row);
        CAGrid {
            size,
            contents,
        }
    }
    fn get(&self, pos_r: &usize, pos_c: &usize) -> Option<Ref<u8>> {
        match self.contents.get(*pos_r) {
            Some(states) =>  match states.get(*pos_c) { Some(state) => Some(state.borrow()), None => None },
            None         =>  None,
        }
    }
    fn get_mut(&self, pos_r: &usize, pos_c: &usize) -> Option<RefMut<u8>> {
        match self.contents.get(*pos_r) {
            Some(states) =>  match states.get(*pos_c) { Some(state) => Some(state.borrow_mut()), None => None },
            None         =>  None,
        }
    }
}
