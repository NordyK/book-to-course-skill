use std::fmt;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoolCreationError {
    ZeroThreads,
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PoolCreationError::ZeroThreads => write!(f, "a thread pool needs at least one thread"),
        }
    }
}

impl std::error::Error for PoolCreationError {}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::ZeroThreads);
        }
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let workers = (0..size).map(|id| Worker::new(id, Arc::clone(&receiver))).collect();
        Ok(ThreadPool { workers, sender })
    }

    pub fn new(size: usize) -> ThreadPool {
        Self::build(size).unwrap_or_else(|e| panic!("{e}"))
    }

    pub fn size(&self) -> usize {
        self.workers.len()
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).expect("workers are running while the pool exists");
    }
}

#[allow(dead_code)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => job(),
                Err(_) => break,
            }
        });
        Worker { id, thread }
    }
}

#[cfg(test)]
mod tests;
