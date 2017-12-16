pub struct Node<T: 'static> {
    a: T,
    d: Option<&'static Node<T>>,
}
impl Node<T> {
    fn car<T>(&self) -> T {
        &self.a
    }
    fn cdr<T>(&self) -> Option<&Node<T>> {
        match self.d {
            Option::Some(i)   =>  i,
            Option::None      =>  None,
        }
    }
}
fn cons<T>(x: &mut Node<T>, y: &Node<T>) {
    let mut end: &Node<T> = x.cdr();
    x = Node {
        a: x.car(),
        d: while end != Option::None {
            match end.cdr() {
                Option::None    =>  y,
                Option::Some(i) =>  end = i,
            }
        }
    }
}
