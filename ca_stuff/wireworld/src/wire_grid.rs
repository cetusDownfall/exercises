use std::rc::{Weak, Rc};
use wire_cells::{WireCell, WCell, WState};
fn holds<T: PartialOrd + Copy + PartialEq + From<i64>>(t1: (T, T), t2: (T, T)) -> bool {
    t1.0 >= 0.into()
 && t1.0 <= t2.0 
 && t1.1 >= 0.into()
 && t1.1 <= t2.1
}
pub trait WGrid<C: WCell> {
    type Prt: Copy + PartialEq + PartialOrd + From<i64> + Into<i64>;
    fn get_cells(&self) -> &[Rc<C>];
    fn put_cell(&mut self, cl: C);
    fn rem_cell(&mut self, pos: (Self::Prt, Self::Prt));
    fn bounds(&self) -> (Self::Prt, Self::Prt);
    fn cell(&self, pos: (Self::Prt, Self::Prt)) -> Option<Rc<C>>;
    fn det_next_state(&self);
    fn next_grid_state(&self) {
        for cell in self.get_cells() {
            cell.to_next();
        }
        self.det_next_state();
    }
    fn add_cell(&mut self, cl: C) {
        if holds(cl.pos(), (self.bounds().0.into(), self.bounds().1.into())) {
            self.put_cell(cl);
        }
    }
    fn put(&mut self, cl: Option<C>, pos: (Self::Prt, Self::Prt)) {
        match cl {
            Some(wc) => self.add_cell(wc),
            None     => self.rem_cell(pos),
        }
    }
}
pub struct WireGrid {
    conts: Vec<Vec<Option<Rc<WireCell>>>>,
    cells: Vec<Rc<WireCell>>,
    bounds: (i64, i64),
}
impl WireGrid {
    pub fn new(bounds: (i64, i64)) -> WireGrid {
        WireGrid {
            conts: 
                vec!
                [ 
                    vec![None; bounds.1 as usize];
                    bounds.0 as usize
                ],
            cells: Vec::new(),
            bounds,
        }
    }
    pub fn refresh_neighbors(&self, wc: Rc<WireCell>) {
        let (r, c) = wc.pos();
        let tmp = 
        (0..9i64).map(|n| (n/3, n % 3)).filter(|&(row, col)| !(row == 1 && col == 1))
            .map(|(row, col)| (r+(row-1), c+(col-1)))
            .filter_map(|p| self.cell(p))
            .map(|n| Rc::downgrade(&n)).collect::<Vec<Weak<WireCell>>>();
        wc.cache_neighbors(tmp);
    }
    pub fn refresh_all(&self) {
        for wc in &self.cells {
            self.refresh_neighbors(Rc::clone(&wc));
        }
    }
    pub fn reset_cells(&mut self) {
        self.cells.drain(..);
        for row in &self.conts {
            for col in row {
                if let Some(ref cl) = *col {
                    self.cells.push(Rc::clone(&cl));
                }
            }
        }
    }
}
impl WGrid<WireCell> for WireGrid {
    type Prt = i64;
    fn get_cells(&self) -> &[Rc<WireCell>] {
        self.cells.as_slice()
    }
    fn put_cell(&mut self, cl: WireCell) {
        let (r, c) = cl.pos();
        let cl = Rc::new(cl);
        self.cells.push(Rc::clone(&cl));
        self.conts[r as usize][c as usize] = Some(Rc::clone(&cl));
        self.refresh_all();
    }
    fn rem_cell(&mut self, pos: (Self::Prt, Self::Prt)) {
        if holds(pos, self.bounds) {
            self.conts[pos.1 as usize][pos.0 as usize] = None;
        }
        self.reset_cells();
    }
    fn bounds(&self) -> (Self::Prt, Self::Prt) {
        self.bounds
    }
    fn cell(&self, pos: (Self::Prt, Self::Prt)) -> Option<Rc<WireCell>> {
        if holds(pos, self.bounds) {
            if let Some(ref r) = self.conts[pos.0 as usize][pos.1 as usize] 
            { Some(Rc::clone(&r)) } else { None } } else { None }
    }
    fn det_next_state(&self) {
        for wc in &self.cells {
            wc.det_next();
        }
    }
}
