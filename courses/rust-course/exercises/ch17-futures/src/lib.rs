use std::future::{Future, poll_fn};
use std::pin::{Pin, pin};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

pub fn block_on<F: Future>(fut: F) -> F::Output {
    todo!()
}

pub struct YieldN {
    pub left: u32,
}

// impl Future for YieldN

type Shared = Arc<Mutex<(bool, Option<Waker>)>>;

pub struct Flag {
    state: Shared,
}

pub struct Setter {
    state: Shared,
}

impl Flag {
    pub fn new() -> (Flag, Setter) {
        todo!()
    }
}

impl Setter {
    pub fn set(&self) {
        todo!()
    }
}

// impl Future for Flag

pub fn join<A: Future, B: Future>(a: A, b: B) -> impl Future<Output = (A::Output, B::Output)> {
    async {
        let result: (A::Output, B::Output) = todo!();
        result
    }
}

#[cfg(test)]
mod tests;
