extern crate and_around;
use and_around::*;
use std::rc::Rc;
fn main() {
    let names = vec!["A", "B", "C", "D", "E", "F"];
    let mut gm = 
        GroupManager::new(names.iter()
                               .map(|&n| Rc::new(Student::new(String::from(n))))
                               .collect::<Vec<_>>(), 16);
    for _ in 0..10 {
        gm.generate_groups(&vec![3;2]);
        println!("{}", gm.show_curr());
        gm.push_curr();
    }
}
