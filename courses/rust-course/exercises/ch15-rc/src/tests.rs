use super::*;

fn setup() -> (Rc<RefCell<HashMap<String, u32>>>, Rc<Recorder>) {
    (Rc::new(RefCell::new(HashMap::new())), recorder())
}

#[test]
fn restock_and_sell() {
    let (stock, rec) = setup();
    let s = Store::new(&stock, rec.clone());
    s.restock("pen", 10);
    assert_eq!(s.sell("pen", 4), Ok(()));
    assert_eq!(stock.borrow()["pen"], 6);
    assert!(rec.messages.borrow().is_empty(), "6 left is not low");
}

#[test]
fn not_enough() {
    let (stock, rec) = setup();
    let s = Store::new(&stock, rec.clone());
    s.restock("ink", 2);
    assert_eq!(s.sell("ink", 3), Err(String::from("not enough ink")));
    assert_eq!(s.sell("paper", 1), Err(String::from("not enough paper")));
    assert_eq!(stock.borrow()["ink"], 2, "unchanged after a failed sale");
}

#[test]
fn low_stock_notifies() {
    let (stock, rec) = setup();
    let s = Store::new(&stock, rec.clone());
    s.restock("pen", 5);
    s.sell("pen", 3).unwrap();
    s.sell("pen", 2).unwrap();
    assert_eq!(*rec.messages.borrow(), vec!["low stock: pen (2)", "low stock: pen (0)"]);
}

#[test]
fn branches_share_stock() {
    let (stock, rec) = setup();
    let north = Store::new(&stock, rec.clone());
    let south = Store::new(&stock, rec.clone());
    north.restock("mug", 4);
    assert_eq!(south.sell("mug", 4), Ok(()));
    assert_eq!(north.sell("mug", 1), Err(String::from("not enough mug")));
    assert_eq!(Rc::strong_count(&stock), 3, "the map has three owners");
}
