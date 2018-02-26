// Author: Ryugo Okada
// Feb. 13, 2018
// Groupings
/*! A command line utility to provide subgroups biased towards 
  separating previously together individuals.
  */
pub mod group;
pub mod student;
pub mod subgroup;
pub use group::Group;
pub use student::Student;
pub use subgroup::Subgroup;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn print_class() {
    }
}
