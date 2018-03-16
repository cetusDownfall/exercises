//yeah i'll get around to this eventually...
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Side {
    Black,
    White,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}
#[derive(Debug)]
pub struct Board {
    tiles: Vec<Piece>,
    side: Side,
}
impl Board {
    pub fn new() -> Board {
        Board {
            tiles: vec![Piece::new(Side::Black, 0, 0),
                        Piece::new(Side::Black, 0, 1),
                        Piece::new(Side::Black, 0, 2),
                        Piece::new(Side::White, 2, 0),
                        Piece::new(Side::White, 2, 1),
                        Piece::new(Side::White, 2, 2)],
            side: Side::White,
        }
    }
    pub fn switch_side(&mut self) {
        if self.side == Side::White {
            self.side = Side::Black;
        } else {
            self.side = Side::White;
        }
    }
    pub fn piece_at_side(&self, row: isize, col: isize) -> Option<&Piece> {
        self.tiles.iter().find(|&&t| t.row() == row && t.col() == col && t.side() == self.side) 
    }
    pub fn piece_at_mut(&mut self, row: isize, col: isize) -> Option<&mut Piece> {
        self.tiles.iter_mut().find(|& &mut t| t.row()==row && t.col()==col)
    }
    pub fn piece_at(&self, row: isize, col: isize) -> Option<&Piece> {
        self.tiles.iter().find(|&&t| t.row()==row && t.col()==col)
    }
    pub fn side(&self) -> Side {
        self.side
    }
    pub fn advance(&mut self, row: isize, col: isize) -> Result<bool,String> {
        let mut va = false;
        if let Some(p) = self.piece_at_side(row, col) {
            if self.piece_at(p.ahead(), col).is_none() {
                va = true;
            } else {
                return Err(String::from("Tile ahead is not empty"));
            }
        } else {
            return Err(String::from("No piece at position given"));
        }
        if va {
            let p = self.piece_at_mut(row, col).unwrap();
            p.adv();
            return Ok(row == 1);
        } else {
            return Err(String::from("Unknown error"));
        }
    }
    pub fn simple_board(&self) -> String {
        let mut out = String::new();
        for r in 0..3 {
            for c in 0..3 {
                out.push(
                match self.piece_at(r, c) {
                    Some(p) => match p.side() { Side::White => 'w', Side::Black => 'b' },
                    None => ' ', 
                });
            }
            out.push('\n');
        }
        out
    }
    pub fn capture(&mut self, row: isize, col: isize, dir: Direction) -> Result<bool,String> {
        let mut va = false;
        let mut rmind = 9;
        if let Some(p) = self.piece_at_side(row, col) {
            if let Some(opp) = self.piece_at(p.ahead(), 
                 match dir { 
                     Direction::Left => p.col() - 1, 
                     Direction::Right => p.col() + 1,
                 }) {
                if opp.side() == p.side() {
                    return Err(String::from("Can't capture allies"));
                } else {
                    va = true;
                    rmind = 
                    self.tiles.iter()
                        .position(|&op| op.row() == opp.row() && op.col() == opp.col()).unwrap();
                }
            } else { return Err(String::from("No piece to capture")); }
        } else { return Err(String::from("No piece at position given")); }
        if rmind != 9 {
            self.tiles.remove(rmind);
        }
        if va {
            let p = self.piece_at_mut(row, col).unwrap();
            p.adv();
            match dir {
                Direction::Left => p.dec_col(),
                Direction::Right => p.inc_col(),
            }
            Ok(row == 1)
        } else { Err(String::from("Unknown error")) }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Piece {
    side: Side,
    row: isize,
    col: isize,
}
impl Piece {
    pub fn new(side: Side, 
               row: isize, 
               col: isize)
        -> Piece 
    {
        Piece { side, row, col }
    }
    pub fn ahead(&self) -> isize { if self.side() == Side::White { self.row - 1 } else { self.row + 1 } }
    pub fn row(&self) -> isize { self.row }
    pub fn col(&self) -> isize { self.col }
    pub fn side(&self) -> Side { self.side }
    pub fn adv(&mut self) {
        if self.side() == Side::White {
            self.row -= 1;
        } else {
            self.row += 1;
        }
    }
    pub fn inc_col(&mut self) { self.col += 1 }
    pub fn dec_col(&mut self) { self.col -= 1 }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
