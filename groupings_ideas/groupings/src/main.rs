extern crate groupings;
use groupings::Group;
fn main() {
    let mut class = 
        Group::new( vec!["Alice", "Bob", "Carol"]);
    class.add_student("Dick");
    class.add_student("Evan");
    class.add_student("Harry");
    class.add_sub(3);
    class.add_sub(3);
    class.fill_subs();
    let test = class.finalise();
    println!("{}", test);
}
