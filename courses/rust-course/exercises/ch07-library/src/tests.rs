use crate::catalog::Item;
use crate::loans::borrow;
use crate::stock_report;

#[test]
fn new_items() {
    let i = Item::new("Dune", 2);
    assert_eq!(i.title, "Dune");
    assert_eq!(i.copies(), 2);
}

#[test]
fn borrowing() {
    let mut i = Item::new("Dune", 1);
    assert_eq!(borrow(&mut i), Ok(()));
    assert_eq!(i.copies(), 0);
    assert_eq!(borrow(&mut i), Err(String::from("no copies of Dune")));
    assert_eq!(i.copies(), 0, "a failed loan changes nothing");
}

#[test]
fn report() {
    let items = [Item::new("Dune", 2), Item::new("Emma", 0)];
    assert_eq!(stock_report(&items), "Dune: 2\nEmma: 0");
    assert_eq!(stock_report(&[]), "", "no items, empty report");
}
