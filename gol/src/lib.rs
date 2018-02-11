pub mod rule;
pub mod cell;
pub mod grid;
pub mod anim;
pub mod input;
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum State {
    Alive,
    Dead,
}
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Action {
    Survive,
    Be,
    Die,
}
#[derive(Hash, Eq, PartialEq, Debug)]
type Position = (u32, u32);
pub trait Neighbor {
    fn count_neighbors(&self, world: grid::Grid) -> u8;
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
