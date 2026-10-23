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
        assert!(size > 0, "a thread pool needs at least one thread");
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        let completed = Arc::new(AtomicUsize::new(0));
        let panicked = Arc::new(AtomicUsize::new(0));
        let workers = (0..size)
            .map(|_| {
                let receiver = Arc::clone(&receiver);
                let completed = Arc::clone(&completed);
                let panicked = Arc::clone(&panicked);
                thread::spawn(move || loop {
                    let message = receiver.lock().unwrap().recv();
                    let Ok(job) = message else { break };
                    match panic::catch_unwind(AssertUnwindSafe(job)) {
                        Ok(()) => completed.fetch_add(1, Ordering::SeqCst),
                        Err(_) => panicked.fetch_add(1, Ordering::SeqCst),
                    };
                })
            })
            .collect();
        ThreadPool { workers, sender: Some(sender), completed, panicked }
    }

    pub fn execute<F>(&self, f: F) -> Result<(), PoolClosed>
    where
        F: FnOnce() + Send + 'static,
    {
        let sender = self.sender.as_ref().ok_or(PoolClosed)?;
        sender.send(Box::new(f)).map_err(|_| PoolClosed)
    }

    pub fn completed(&self) -> usize {
        self.completed.load(Ordering::SeqCst)
    }

    pub fn panicked(&self) -> usize {
        self.panicked.load(Ordering::SeqCst)
    }

    pub fn shutdown(&mut self) {
        drop(self.sender.take());
        for worker in self.workers.drain(..) {
            worker.join().expect("workers catch job panics, so they do not panic themselves");
        }
    }

    pub fn join(mut self) -> (usize, usize) {
        self.shutdown();
        (self.completed(), self.panicked())
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.shutdown();
    }
}

#[cfg(test)]
mod tests;
