use super::*;

fn new_log() -> Rc<RefCell<Vec<String>>> {
    Rc::new(RefCell::new(Vec::new()))
}

#[test]
fn derefs_to_inner() {
    let log = new_log();
    let t = Tracked::new("x", String::from("hello"), &log);
    assert_eq!(t.len(), 5, "method call through Deref");
    assert_eq!(*t, "hello");
}

#[test]
fn deref_mut_changes_inner() {
    let log = new_log();
    let mut v = Tracked::new("v", vec![1, 2], &log);
    v.push(3);
    *v = vec![9];
    assert_eq!(*v, vec![9]);
}

#[test]
fn logs_create_and_drop_in_order() {
    let log = new_log();
    {
        let _a = Tracked::new("a", 1, &log);
        let _b = Tracked::new("b", 2, &log);
    }
    assert_eq!(*log.borrow(), vec!["create a", "create b", "drop b", "drop a"]);
}

#[test]
fn early_drop() {
    let log = new_log();
    let a = Tracked::new("a", 1, &log);
    drop(a);
    log.borrow_mut().push(String::from("after"));
    assert_eq!(*log.borrow(), vec!["create a", "drop a", "after"]);
}

#[test]
fn coercion_to_str() {
    let log = new_log();
    let x = Tracked::new("x", String::from("abc"), &log);
    let y = Tracked::new("y", String::from("de"), &log);
    assert_eq!(total_len(&[&x, &y, "f"]), 6, "&Tracked<String> coerces to &str");
}
