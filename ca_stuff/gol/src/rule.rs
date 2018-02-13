pub enum State {
    Alive,
    Dead,
}
pub enum Action {
    Survive,
    Be,
    Die,
}
pub struct Rule {
    survive: &[u8],
    be: &[u8],
    die: &[u8]
}
impl Rule {
    pub fn get_action(cell: Cell) -> Action {
    }
}
