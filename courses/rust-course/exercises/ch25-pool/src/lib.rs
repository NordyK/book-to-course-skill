use std::fmt;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoolCreationError {
    ZeroThreads,
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for PoolCreationError {}

pub struct ThreadPool {
    // TODO: the workers and the sending half of the job channel
}

impl ThreadPool {
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        todo!()
    }

    pub fn new(size: usize) -> ThreadPool {
        todo!()
    }

    pub fn size(&self) -> usize {
        todo!()
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests;
