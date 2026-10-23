use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub trait Notifier {
    fn notify(&self, msg: &str);
}

pub struct Store {
    stock: Rc<RefCell<HashMap<String, u32>>>,
    notifier: Rc<dyn Notifier>,
}

impl Store {
    pub fn new(stock: &Rc<RefCell<HashMap<String, u32>>>, notifier: Rc<dyn Notifier>) -> Store {
        todo!()
    }

    pub fn restock(&self, item: &str, n: u32) {
        todo!()
    }

    pub fn sell(&self, item: &str, n: u32) -> Result<(), String> {
        todo!()
    }
}

pub struct Recorder {
    pub messages: RefCell<Vec<String>>,
}

// impl Notifier for Recorder

pub fn recorder() -> Rc<Recorder> {
    todo!()
}

#[cfg(test)]
mod tests;
