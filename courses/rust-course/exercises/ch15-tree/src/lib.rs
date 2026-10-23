use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Node {
    pub name: String,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn new_node(name: &str) -> Rc<Node> {
    todo!()
}

pub fn add_child(parent: &Rc<Node>, child: &Rc<Node>) {
    todo!()
}

pub fn parent_name(node: &Node) -> Option<String> {
    todo!()
}

pub fn path(node: &Rc<Node>) -> String {
    todo!()
}

pub fn count(node: &Rc<Node>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests;
