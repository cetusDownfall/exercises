use std::collections::HashSet;
pub struct Grid {
    conts: HashSet<Cell>,
}
impl Grid {
    pub fn new(size: u32) -> Grid {
        let points: HashMap<Position, State> = HashMap::new();
        (0..).take(size).flat_map(|n| (0..).take(n).map(|m| (n, m))).for_each(|p| points.insert(p, State::Dead));
        Grid {
            conts: points,
        }
    }
    pub fn get_cell(&self, pos: Position) -> Option<Cell> {
        self.conts.get(pos)
    }
}
