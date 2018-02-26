//! Module containing the subgroup struct and methods for it.

use std::rc::*;
use std::cell::RefCell;
use student::*;
type Weight = u32;
/// A small group of students. The second member is a max size.
pub struct Subgroup(Vec<Rc<RefCell<Student>>>,usize);
impl Subgroup {
    /// Initialises an empty subgroup.
    pub fn new(max_size: usize) -> Subgroup {
        Subgroup(Vec::new(), max_size)
    }
    /// Gives the amount of times people in this group have been matched with an individual.
    pub fn weight(&self, p: &Student) -> Weight {
        self.0.iter().filter_map(|ref s| s.borrow().get_weight(p)).fold(0u32, |acc, n| acc+n)
    }
    /// Takes a student and adds it to the group. Doesn't increment weights.
    /// Doesn't check for full capacity.
    fn add_student(&mut self, p: Rc<RefCell<Student>>) {
        self.0.push(Rc::clone(&p));
    }
    /// Takes the current students in the subgroup and increments their weights for each 
    /// individual in the group by 1.
    pub fn inc_weights(&mut self) {
        let inds = self.0.iter().map(|s| s.borrow().id()).collect::<Vec<usize>>();
        for s in self.0.iter() {
            for n in inds.iter() {
                s.borrow_mut().inc_weight(*n);
            }
        }
    }
    /// Get the max size of a subgroup.
    fn max_size(&self) -> usize {
        self.1
    }
    /// Check how many individuals there are in this group.
    fn student_ct(&self) -> usize {
        self.0.len()
    }
    /// Check number of remaining spots.
    fn remaining(&self) -> usize {
        self.max_size() - self.student_ct()
    }
    /// Adds a student if not full. Returns a tuple with the number of spots remaining and the
    /// student if it was full. If it wasn't full, then it returns none instead of a student.
    pub fn try_add(&mut self, p: Option<Rc<RefCell<Student>>>) -> usize {
        if let Some(p) = p {
            if self.remaining() > 0 {
                self.add_student(p);
                self.remaining()
            } else {
                self.remaining()
            }
        } else {
            0
        }
    }
    /// Access the names of the students in this subgroup.
    pub fn list_students(&self) -> Vec<String> {
        self.0.iter().map(|ref s| String::from(s.borrow().name())).collect::<Vec<String>>()
    }
}
