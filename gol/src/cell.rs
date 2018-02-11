#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Cell {
    state: State,
    position: Position,
}
