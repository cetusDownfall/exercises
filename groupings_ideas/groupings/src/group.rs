//! Contains group and methods for it.
/// Main group of individuals.
/// Holds subgroups, can have a name
use std::rc::*;
use std::cell::RefCell;
use subgroup::*;
use student::*;
pub struct Group {
    subgroups: Vec<Subgroup>,
    students: Vec<Rc<RefCell<Student>>>,
    name: Option<String>,
}
impl Group {
    /// Constructor taking a vector of strings and uses those as names for students.
    pub fn new(names: Vec<&str>) -> Group {
        let mut students: Vec<Rc<RefCell<Student>>> = Vec::new();
        let length = names.len();
        for (i, n) in names.into_iter().enumerate() {
            students.push(Rc::new(RefCell::new(Student::new(i,n,length))));
        }
        Group {
            subgroups: Vec::new(),
            students,
            name: None,
        }
    }
    /// Name setter.
    pub fn set_name(&mut self, s: &str) {
        self.name = Some(String::from(s));
    }
    /// Remove a subgroup at an index and return all students in it to the group.
    pub fn remove_sub(&mut self, ind: usize) {
        if ind < self.subgroups.len() {
            self.subgroups.remove(ind);
        }
    }
    /// Add a subgroup of a specific size.
    pub fn add_sub(&mut self, size: usize) {
        self.subgroups.push(Subgroup::new(size));
    }
    /// Evaluate what students should be in what groups.
    /// (only use after creating groups with add_sub)
    pub fn make_groups(&mut self) {
    }
    /// Add a student
    pub fn add_student(&mut self, name: &str) {
        let old_len = self.students.len();
        self.students.push(Rc::new(RefCell::new(Student::new(old_len, name, old_len + 1))));
        for s in self.students.iter() {
            s.borrow_mut().add_slot();
        }
    }
    /// Clear subgroups vec.
    pub fn clear_subgroups(&mut self) {
        self.subgroups.clear();
    }
    /// Temporary. Fills subgroups.
    pub fn fill_subs(&mut self) {
        let mut sts = self.students.iter();
        for sgroup in self.subgroups.iter_mut() {
            while sgroup.try_add(
                if let Some(n) = sts.next() {
                    Some(Rc::clone(&n))
                } else {
                    None
                }) > 0 {}
        }
    }
    /// Finalise subgroups. Increment all weights.
    pub fn finalise(&mut self) -> String {
        let mut out = String::new();
        for (i, sgroup) in self.subgroups.iter_mut().enumerate() {
            sgroup.inc_weights();
            out = out + &format!("\nSubgroup {}:", i);
            for s in sgroup.list_students() {
                out = out + &format!("\n\t{}", s);
            }
            out = out + "\n";
        }
        out
    }
}
