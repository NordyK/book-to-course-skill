use super::*;
use std::time::Duration;

#[test]
fn runs_plain_async() {
    assert_eq!(block_on(async { 40 + 2 }), 42);
}

#[test]
fn yields() {
    assert_eq!(block_on(async { YieldN { left: 3 }.await; 7 }), 7);
    assert_eq!(block_on(async { YieldN { left: 0 }.await; 1 }), 1);
}

#[test]
fn flag_set_from_another_thread() {
    let (flag, setter) = Flag::new();
    let t = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        setter.set();
    });
    block_on(flag);
    t.join().unwrap();
}

#[test]
fn flag_already_set() {
    let (flag, setter) = Flag::new();
    setter.set();
    block_on(flag);
}

#[test]
fn joins_concurrently() {
    assert_eq!(block_on(join(async { 1 }, async { "two" })), (1, "two"));

    // the second future can only finish if the first is polled concurrently with it
    let (flag, setter) = Flag::new();
    let first = async move {
        YieldN { left: 2 }.await;
        setter.set();
        "set"
    };
    let second = async move {
        flag.await;
        "saw it"
    };
    assert_eq!(block_on(join(second, first)), ("saw it", "set"));
}
