use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct Tracked<T> {
    value: T,
    log: Rc<RefCell<Vec<String>>>,
    name: String,
}

impl<T> Tracked<T> {
    pub fn new(name: &str, value: T, log: &Rc<RefCell<Vec<String>>>) -> Self {
        log.borrow_mut().push(format!("create {name}"));
        todo!()
    }
}

// impl<T> Deref for Tracked<T>

// impl<T> DerefMut for Tracked<T>

// impl<T> Drop for Tracked<T>

pub fn total_len(items: &[&str]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests;
