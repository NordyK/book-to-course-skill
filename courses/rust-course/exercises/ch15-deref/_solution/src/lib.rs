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
        Tracked { value, log: Rc::clone(log), name: name.to_string() }
    }
}

impl<T> Deref for Tracked<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> DerefMut for Tracked<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> Drop for Tracked<T> {
    fn drop(&mut self) {
        self.log.borrow_mut().push(format!("drop {}", self.name));
    }
}

pub fn total_len(items: &[&str]) -> usize {
    items.iter().map(|s| s.len()).sum()
}

#[cfg(test)]
mod tests;
