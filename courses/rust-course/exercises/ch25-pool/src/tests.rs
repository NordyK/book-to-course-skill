use super::*;
use std::collections::HashSet;
use std::sync::Barrier;
use std::time::Duration;

#[test]
fn zero_is_an_error() {
    assert_eq!(ThreadPool::build(0).err(), Some(PoolCreationError::ZeroThreads));
    assert_eq!(PoolCreationError::ZeroThreads.to_string(), "a thread pool needs at least one thread");
    assert_eq!(ThreadPool::build(3).unwrap().size(), 3);
    assert_eq!(ThreadPool::new(1).size(), 1);
}

#[test]
#[should_panic]
fn new_zero_panics() {
    ThreadPool::new(0);
}

#[test]
fn runs_every_job() {
    let pool = ThreadPool::new(4);
    let (tx, rx) = mpsc::channel();
    for n in 0..50 {
        let tx = tx.clone();
        pool.execute(move || tx.send(n).unwrap());
    }
    drop(tx);
    let mut got: Vec<i32> = rx.iter().take(50).collect();
    got.sort();
    assert_eq!(got, (0..50).collect::<Vec<_>>());
}

#[test]
fn jobs_run_on_pool_threads() {
    let pool = ThreadPool::new(3);
    let (tx, rx) = mpsc::channel();
    for _ in 0..30 {
        let tx = tx.clone();
        pool.execute(move || {
            thread::sleep(Duration::from_millis(2));
            tx.send(thread::current().id()).unwrap();
        });
    }
    let ids: HashSet<_> = (0..30).map(|_| rx.recv_timeout(Duration::from_secs(5)).unwrap()).collect();
    assert!(ids.len() <= 3, "used {} threads", ids.len());
    assert!(!ids.contains(&thread::current().id()), "jobs must not run on the caller's thread");
}

#[test]
fn runs_in_parallel() {
    // every job waits until all four are running; a pool that runs one job at a time never gets there
    let pool = ThreadPool::new(4);
    let barrier = Arc::new(Barrier::new(4));
    let (tx, rx) = mpsc::channel();
    for _ in 0..4 {
        let (barrier, tx) = (Arc::clone(&barrier), tx.clone());
        pool.execute(move || {
            barrier.wait();
            tx.send(()).unwrap();
        });
    }
    for _ in 0..4 {
        rx.recv_timeout(Duration::from_secs(5)).expect("jobs did not run at the same time");
    }
}

#[test]
fn closures_can_own_data() {
    let pool = ThreadPool::new(2);
    let (tx, rx) = mpsc::channel();
    let data = vec![1, 2, 3];
    let name = String::from("sum");
    pool.execute(move || tx.send(format!("{name} = {}", data.iter().sum::<i32>())).unwrap());
    assert_eq!(rx.recv_timeout(Duration::from_secs(5)).unwrap(), "sum = 6");
}
