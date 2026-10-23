use std::panic::{self, AssertUnwindSafe};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PoolClosed;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: Option<mpsc::Sender<Job>>,
    completed: Arc<AtomicUsize>,
    panicked: Arc<AtomicUsize>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        todo!()
    }

    pub fn execute<F>(&self, f: F) -> Result<(), PoolClosed>
    where
        F: FnOnce() + Send + 'static,
    {
        todo!()
    }

    pub fn completed(&self) -> usize {
        todo!()
    }

    pub fn panicked(&self) -> usize {
        todo!()
    }

    pub fn shutdown(&mut self) {
        todo!()
    }

    pub fn join(mut self) -> (usize, usize) {
        todo!()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // TODO
    }
}

#[cfg(test)]
mod tests;
