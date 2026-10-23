use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Node {
    pub name: String,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn new_node(name: &str) -> Rc<Node> {
    Rc::new(Node {
        name: name.to_string(),
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(Vec::new()),
    })
}

pub fn add_child(parent: &Rc<Node>, child: &Rc<Node>) {
    parent.children.borrow_mut().push(Rc::clone(child));
    *child.parent.borrow_mut() = Rc::downgrade(parent);
}

pub fn parent_name(node: &Node) -> Option<String> {
    node.parent.borrow().upgrade().map(|p| p.name.clone())
}

pub fn path(node: &Rc<Node>) -> String {
    let mut names = vec![node.name.clone()];
    let mut current = Rc::clone(node);
    loop {
        let parent = current.parent.borrow().upgrade();
        match parent {
            Some(p) => {
                names.push(p.name.clone());
                current = p;
            }
            None => break,
        }
    }
    names.reverse();
    names.join("/")
}

pub fn count(node: &Rc<Node>) -> usize {
    1 + node.children.borrow().iter().map(count).sum::<usize>()
}

#[cfg(test)]
mod tests;
