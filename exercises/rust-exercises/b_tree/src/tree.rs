use std::rc::*;
use std::cell::RefCell;
use std::cmp::PartialOrd;
pub struct Node<T: PartialOrd> {
    conts: Box<T>,
    children: [RefCell<Option<Rc<Node<T>>>>; 2],
    parent: RefCell<Option<Weak<Node<T>>>>
}
impl<T: PartialOrd> Node<T> {
    pub fn new(conts: T) -> Node<T> {
        Node {
            conts: Box::new(conts),
            children: [RefCell::new(None); 2],
            parent: RefCell::new(None)
        }
    }
    pub fn left(&self) -> Option<Rc<Node<T>>> {
        match self.children[0].borrow() {
            ref Some(l) => Some(Rc::clone(l)),
            ref None => None
        }
    }
    pub fn right(&self) -> Option<Rc<Node<T>>> {
        match self.children[1].borrow() {
            ref Some(r) => Some(Rc::clone(r)),
            ref None => None
        }
    }
    pub fn parent(&self) -> Option<Rc<Node<T>>> {
        match self.parent.borrow() {
            ref Some(p) => {
                match Rc::upgrade(p) {
                    Some(n) => Some(n),
                    None => None } },
            ref None => None
        }
    }
    pub fn set_left(&self, child: &Rc<Node<T>>) {
        *self.children[0].borrow_mut() = Some(Rc::clone(child));
    }
    pub fn set_right(&self, child: &Rc<Node<T>>) {
        *self.children[1].borrow_mut() = Some(Rc::clone(child));
    }
    pub fn set_parent(&self, parent: &Rc<Node<T>>) {
        *self.parent.borrow_mut() = Some(Rc::downgrade(parent));
    }
    pub fn insert(&self, value: T) {
        if value < *self.conts {
            if let Some(n) = self.left() {
                n.insert(value);
            } else {
                let d = Node::new(val);
                d.set_parent(&Rc::new(self));
                self.set_left(&Rc::new(d));
            }
        } else if value > *self.conts {
            if let Some(n) = self.right() {
                n.insert(value);
            } else {
                let d = Node::new(val);
                d.set_parent(&Rc::new(self));
                self.set_right(&Rc::new(d));
            }
        }
    }
}
