//! Holds a revolving list of a fixed amount of reference counted items.
use std::rc::Rc;
/// A list of fixed length represented by a mutable slice of owned slices.
pub struct RoundList<T> {
    conts: Vec<Rc<T>>,
    size: usize,
}
impl<T> RoundList<T> {
    /// Constructor, takes a size to keep as its max size.
    pub fn new(size: usize) -> RoundList<T> {
        RoundList {
            conts: Vec::new(),
            size,
        }
    }
    /// Inserts this item at the next index. Loops around once it hits the max size.
    pub fn insert(&mut self, item: T) {
        if self.conts.len() < self.size {
            self.conts.insert(0, Rc::new(item));
        } else {
            self.conts.pop();
            self.conts.insert(0, Rc::new(item));
        }
    }
    /// Returns a list with reference counted references to this list's contents.
    pub fn get_list<'a>(&'a self) -> Vec<Rc<T>> {
        self.conts.iter().map(|e| Rc::clone(e))
                  .collect::<Vec<Rc<T>>>()
    }
}
