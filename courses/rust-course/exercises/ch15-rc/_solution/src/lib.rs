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
        Store { stock: Rc::clone(stock), notifier }
    }

    pub fn restock(&self, item: &str, n: u32) {
        *self.stock.borrow_mut().entry(item.to_string()).or_insert(0) += n;
    }

    pub fn sell(&self, item: &str, n: u32) -> Result<(), String> {
        let left = {
            let mut stock = self.stock.borrow_mut();
            let count = stock.get_mut(item).filter(|c| **c >= n).ok_or(format!("not enough {item}"))?;
            *count -= n;
            *count
        };
        if left < 3 {
            self.notifier.notify(&format!("low stock: {item} ({left})"));
        }
        Ok(())
    }
}

pub struct Recorder {
    pub messages: RefCell<Vec<String>>,
}

impl Notifier for Recorder {
    fn notify(&self, msg: &str) {
        self.messages.borrow_mut().push(msg.to_string());
    }
}

pub fn recorder() -> Rc<Recorder> {
    Rc::new(Recorder { messages: RefCell::new(Vec::new()) })
}

#[cfg(test)]
mod tests;
