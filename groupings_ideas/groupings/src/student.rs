//! Module containing student and methods for it.
/// Representation of a student as an id number and a name. The id number is an index, and should
/// not be set manually.
/// The weights contain individual values for each other individual in a group. Again, should not
/// be set manually.
type Weight = u32;
pub struct Student {
    id: usize,
    name: String,
    weights: Vec<Weight>,
}
impl Student {
    /// Returns the ID as a usize
    pub fn id(&self) -> usize {
        self.id
    }
    /// Student constructor. Takes an id, a name, and a number of weight slots to provide.
    pub fn new(id: usize, name: &str, weight_ct: usize) -> Student {
        Student {
            id,
            name: String::from(name),
            weights: vec![0;weight_ct],
        }
    }
    /// Add an index to stor a student weight in.
    pub fn add_slot(&mut self) {
        self.weights.push(0);
    }
    /// Return the weight for a certain student.
    /// None if out of bounds.
    /// Not guaranteed to return the same if args switch order.
    fn weight_for(&self, p: &Student) -> Option<Weight> {
        if let Some(&n) = self.weights.get(p.id()) {
            Some(n)
        } else {
            None
        }
    }
    /// Returns a pair of weights for two students.
    fn weight_pair(p1: &Student, p2: &Student) -> (Option<Weight>, Option<Weight>) {
        (p1.weight_for(p2), p2.weight_for(p1))
    }
    /// Returns the weight of a grouping of p1 and p2, if they are the same.
    /// Otherwise, returns the higher weight or none.
    pub fn get_weight(&self, p2: &Student) -> Option<Weight> {
        match Student::weight_pair(self,p2) {
            (Some(a), Some(b)) => Some(a.max(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None
        }
    }
    /// Setter for weight of a student.
    pub fn set_weight(&mut self, p: &Student, weight: Option<Weight>) {
        if let Some(w) = weight {
            if let Some(n) = self.weights.get_mut(p.id()) {
                *n = w;
            }
        }
    }
    /// Increments weight by 1 at a given index.
    pub fn inc_weight(&mut self, index: usize) {
        if let Some(i) = self.weights.get_mut(index) {
            *i+=1;
        }
    }
    /// Sets a pair of students to a weight.
    pub fn set_pair_weight(p1: &mut Student, p2: &mut Student, weight: Option<Weight>) {
        p1.set_weight(&p2, weight);
        p2.set_weight(&p1, weight);
    }
    /// Access the name field immutably.
    pub fn name(&self) -> String {
        String::clone(&self.name)
    }
}
