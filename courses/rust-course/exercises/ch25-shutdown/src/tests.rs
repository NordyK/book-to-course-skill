use super::*;
use std::time::Duration;

#[test]
#[should_panic]
fn zero_threads_panics() {
    ThreadPool::new(0);
}

#[test]
fn drop_waits_for_every_job() {
    let counter = Arc::new(AtomicUsize::new(0));
    let pool = ThreadPool::new(3);
    for _ in 0..20 {
        let counter = Arc::clone(&counter);
        pool.execute(move || {
            thread::sleep(Duration::from_millis(2));
            counter.fetch_add(1, Ordering::SeqCst);
        })
        .unwrap();
    }
    drop(pool);
    assert_eq!(counter.load(Ordering::SeqCst), 20);
}

#[test]
fn queued_jobs_run_before_shutdown_returns() {
    let mut pool = ThreadPool::new(1);
    pool.execute(|| thread::sleep(Duration::from_millis(50))).unwrap();
    for _ in 0..5 {
        pool.execute(|| {}).unwrap();
    }
    pool.shutdown();
    assert_eq!(pool.completed(), 6);
    assert_eq!(pool.panicked(), 0);
}

#[test]
fn execute_after_shutdown_is_an_error() {
    let mut pool = ThreadPool::new(2);
    pool.shutdown();
    assert_eq!(pool.execute(|| {}), Err(PoolClosed));
    pool.shutdown(); // twice is fine
}

#[test]
fn panicking_jobs_do_not_kill_workers() {
    let pool = ThreadPool::new(2);
    for _ in 0..3 {
        pool.execute(|| panic!("job failed on purpose")).unwrap();
    }
    for _ in 0..5 {
        pool.execute(|| thread::sleep(Duration::from_millis(1))).unwrap();
    }
    assert_eq!(pool.join(), (5, 3));
}

#[test]
fn counts_are_live() {
    let pool = ThreadPool::new(2);
    let (tx, rx) = mpsc::channel();
    pool.execute(move || tx.send(()).unwrap()).unwrap();
    rx.recv_timeout(Duration::from_secs(5)).unwrap();
    let mut waited = 0;
    while pool.completed() < 1 && waited < 500 {
        thread::sleep(Duration::from_millis(1));
        waited += 1;
    }
    assert_eq!(pool.completed(), 1);
}
