use wire_grid::{WGrid};
use std::cell::Cell;
use std::rc::{Weak, Rc};
pub type SharedCell = Rc<WireCell>;

#[derive(Copy, Clone, PartialEq)]
pub enum WState {
    Conductor,
    EHead,
    ETail,
}
pub trait WCell  {
    fn pos(&self) -> (i64, i64);
    fn get_state(&self) -> WState;
    fn get_neighbors(&self) -> Vec<Rc<Self>>;
    fn to_next(&self);
    fn set_next(&self, n: WState);
    fn det_next(&self) {
        match self.get_state() {
            WState::Conductor => {
                let hc: i64 = self.get_neighbors()
                             .iter()
                             .filter(|n| n.get_state() == WState::EHead)
                             .map(|_| 1i64).sum();
                if hc == 1 || hc == 2 {
                    self.set_next(WState::EHead);
                } else {
                    self.set_next(WState::Conductor);
                }
            },
            WState::EHead => self.set_next(WState::ETail),
            WState::ETail => self.set_next(WState::Conductor),
        }
    }
}
pub struct WireCell {
    p: (i64, i64),
    c: Cell<WState>,
    n: Cell<WState>,
    b: Cell<Vec<Weak<WireCell>>>
}
impl WireCell {
    pub fn new(state: WState, row: i64, col: i64) -> WireCell {
        WireCell {
            p: (row, col),
            c: Cell::new(state),
            n: Cell::new(state),
            b: Cell::new(Vec::new())
        }
    }
    pub fn cache_neighbors(&self, ns: Vec<Weak<WireCell>>) {
        self.b.set(ns);
        self.det_next();
    }
}
impl WCell for WireCell {
    fn pos(&self) -> (i64, i64) {
        self.p
    }
    fn get_state(&self) -> WState {
        self.c.get()
    }
    fn get_neighbors(&self) -> Vec<Rc<WireCell>> {
        let ns = self.b.take();
        let out = ns.iter().filter_map(|w| w.upgrade()).collect::<Vec<Rc<WireCell>>>();
        self.b.set(ns);
        out
    }
    fn to_next(&self) {
        self.c.swap(&self.n);
    }
    fn set_next(&self, n: WState) {
        self.n.set(n);
    }
}
