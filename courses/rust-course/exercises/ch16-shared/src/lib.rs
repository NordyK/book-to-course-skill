use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn count_words(texts: Vec<String>, threads: usize) -> (BTreeMap<String, usize>, usize) {
    todo!()
}

pub struct Stats {
    inner: Mutex<Vec<u64>>,
}

impl Stats {
    pub fn new() -> Stats {
        todo!()
    }

    pub fn record(&self, v: u64) {
        todo!()
    }

    pub fn mean(&self) -> Option<f64> {
        todo!()
    }
}

#[cfg(test)]
mod tests;
