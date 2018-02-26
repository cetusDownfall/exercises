//! Contains several structs for group management and record keeping. May be refactored later.
use super::buffer::RoundList;
use std::rc::{Weak, Rc};
/// A wrapper around a string. May be changed later.
pub struct Student(String);
/// A group of students. Owns its students.
pub struct Class(Vec<Rc<Student>>);
/// A temporary group for record keeping, primarily. Weak references so as to allow classes to add
/// and remove students later and still keep past groups with references valid.
pub struct Group(Vec<Weak<Student>>, usize);
/// A group of groups, owns references to them. Used almost entirely for record keeping.
pub struct GroupState(Vec<Rc<Group>>);
/// Contains a circular list of previous groupstates to keep track of student weight.
pub struct History(RoundList<Rc<GroupState>>);
/// Primary struct used for creating, storing, and distributing students among groups.
pub struct GroupManager {
    pool: Class,
    curr: GroupState,
    hist: History,
}
impl Student {
    /// Constructor. Takes a static string reference as an argument.
    pub fn new(name: String) -> Student {
        Student(name)
    }
    pub fn name(&self) -> &String {
        &self.0
    }
}
impl Class {
    /// Constructor. Takes a reference counted list of students (should have no other references to
    /// them.)
    pub fn new(students: Vec<Rc<Student>>) -> Class {
        Class(students)
    }
    /// Returns a list of references to the students inside.
    pub fn get_students(&self) -> Vec<Rc<Student>> {
        self.0.iter()
            .map(|s| Rc::clone(s))
            .collect::<Vec<Rc<Student>>>()
    }
    /// Returns a list of weak references to the students inside. Used primarily for record
    /// keeping.
    pub fn get_weak_students(&self) -> Vec<Weak<Student>> {
        self.0.iter()
              .map(|s| Rc::downgrade(&s))
              .collect::<Vec<Weak<Student>>>()
    }
    /// Shuffles the students in the class.
    pub fn salt(&mut self) {
    }
    /// Rotates the students in the class.
    pub fn rot(&mut self, ind: usize) {
        for _ in 0..ind {
            if let Some(s) = self.0.pop() {
                self.0.insert(0, s);
            }
        }
    }
}
impl Group {
    /// Constructor. Takes a max size.
    pub fn new(size: usize) -> Group {
        Group(Vec::new(), size)
    }
    /// Adds a student unconditionally. Do not use, low level.
    fn add_student(&mut self, student: Rc<Student>) {
        self.0.push(Rc::downgrade(&student));
    }
    /// Adds a student but checks for capacity first. Returns a None if successful, Some(argument)
    /// otherwise.
    pub fn try_add(&mut self, student: Rc<Student>) -> Option<Rc<Student>> {
        self.add_student(student);
        if self.0.len() <= self.1 {
            None
        } else {
            if let Some(w) = self.0.pop() {
                w.upgrade()
            } else {
                None
            }
        }
    }
    /// Checks if a reference in this grou
    pub fn contains(&self, student: Rc<Student>) -> bool {
        self.0
            .iter()
            .fold(false, |acc, w|
                    acc ||
                    if let Some(s) = w.upgrade() {
                        Rc::ptr_eq(&student, &s)
                    } else {
                        false 
                    })
    }
    /// Returns remaining spots for students.
    pub fn remaining(&self) -> usize {
        self.1 - self.0.len()
    }
    /// Lists students.
    pub fn get_students(&self) -> Vec<Rc<Student>> {
        self.0.iter().filter_map(|s| s.upgrade()).collect::<Vec<_>>()
    }
}
impl GroupState {
    /// Constructor. Takes nothing, returns an empty group state.
    pub fn new() -> GroupState {
        GroupState(Vec::new())
    }
    pub fn make_record(&mut self) -> GroupState {
        GroupState(self.0.drain(..).collect::<Vec<_>>())
    }
    /// Adds an already initialised group.
    pub fn add_group(&mut self, group: Rc<Group>) {
        self.0.push(Rc::clone(&group));
    }
    /// Adds an empty group. Avoid if possible.
    pub fn add_new_group(&mut self, size: usize) {
        self.0.push(Rc::new(Group::new(size)))
    }
    /// Returns a list of counted references to the contents of this struct.
    pub fn get_groups(&self) -> Vec<Rc<Group>> {
        self.0.iter().map(|g| Rc::clone(g)).collect::<Vec<_>>()
    }
    /// Returns only the groups which contain both of the students provided.
    pub fn has_both(&self, s1: Rc<Student>, s2: Rc<Student>) -> Vec<Rc<Group>> {
        self.get_groups()
            .into_iter()
            .filter(|g| !(g.contains(Rc::clone(&s1)) && g.contains(Rc::clone(&s2))))
            .collect::<Vec<Rc<Group>>>()
    }
}
impl History {
    /// Constructor. Takes a max length of groupstates to keep track of.
    pub fn new(length: usize) -> History {
        History(RoundList::new(length))
    }
    /// Gets a "bias number" based off of the amount of times the students have been grouped
    /// together and how long ago it was.
    pub fn get_weight(&self, s1: Rc<Student>, s2: Rc<Student>) -> u32 {
        self.0.get_list()
              .into_iter()
              .rev()
              .enumerate()
              .map(|(i, gs)|
                   gs.has_both(Rc::clone(&s1), Rc::clone(&s2))
                     .len() as u32 * (1+i as u32))
              .sum()
    }
    /// Adds a new group state.
    pub fn insert(&mut self, gs: Rc<GroupState>) {
        self.0.insert(gs);
    }
}
impl GroupManager {
    /// Constructor. Takes a list of student references, preferably new. Also takes a length of
    /// group states to keep track of.
    pub fn new(students: Vec<Rc<Student>>, 
               hist_len: usize) -> GroupManager {
        GroupManager { 
            pool: Class::new(students),
            curr: GroupState::new(),
            hist: History::new(hist_len),
        }
    }
    /// Distribute students without bias. Simply pour them out into the number of groups provided,
    /// each with the size provided.
    pub fn distribute(&mut self, sizes: &[usize]) {
        let mut tpool = self.pool.get_students();
        for &z in sizes {
            let mut g = Group::new(z);
            loop {
                if let Some(s) = tpool.pop() {
                    if let Some(r) = g.try_add(s) {
                            tpool.push(r);
                            break; 
                    } else {
                        continue;
                    }
                }
                break;
            }
            self.curr.add_group(Rc::new(g));
        }
    }
    pub fn generate_groups(&mut self, sizes: &[usize]) {
        let mut tpool = self.pool.get_students();
        let mut groups = 
            sizes.iter()
                 .map(|&z| Group::new(z))
                 .collect::<Vec<Group>>();
        while 0usize != groups.iter().map(|g| g.remaining()).sum()
        {
            if let Some(s) = tpool.pop() {
                    let lvals = 
                        groups.iter()
                              .filter(|o| o.remaining() > 0)
                              .filter_map(|u| u.get_students().iter()
                                        .map(|t| self.hist.get_weight(Rc::clone(&s), Rc::clone(&t)))
                                        .min())
                          .collect::<Vec<u32>>();
                {
                    if let Some(mut ming) = 
                        groups.iter_mut()
                          .zip(lvals.iter())
                          .filter(|&(ref g, &lval)| 
                                     g.remaining() > 0
                                  && g.get_students().len() > 0
                                  && g.get_students().iter()
                                      .map(|t| self.hist.get_weight(Rc::clone(&s), Rc::clone(&t)))
                                      .min().unwrap() <= lval)
                          .min_by_key(
                           |&(ref g, _)| -> u32 {
                             g.get_students().iter()
                              .map(|t| self.hist.get_weight(Rc::clone(&s), Rc::clone(&t)))
                                                       .sum()}) 
                    {
                        ming.0.try_add(Rc::clone(&s));
                    }
                }
            } else {
                break;
            }
        }
        groups.into_iter().for_each(|g| self.curr.add_group(Rc::new(g)));
    }
    pub fn push_curr(&mut self) {
        self.pool.rot(17);
        self.hist.insert(Rc::new(self.curr.make_record()));
    }
    /// Shows the current group state.
    pub fn show_curr(&self) -> String {
        let mut out = String::new();
        let gs = self.curr.get_groups();
        for (i, g) in gs.iter().enumerate() {
            out = out + "\n" + &format!("Group {}: \n", i);
            let ss = g.get_students();
            for s in &ss {
                out = out + s.name() + " ";
            }
            out = out + "\n";
        }
        out + "\n"
    }
}
