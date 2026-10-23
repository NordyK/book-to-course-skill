use super::*;

fn dune() -> Book {
    new_book(String::from("Dune"), String::from("Frank Herbert"), Isbn(String::from("978-0441013593")), 1965)
}

#[test]
fn new_books_are_available() {
    let b = dune();
    assert_eq!(b.title, "Dune");
    assert_eq!(b.author, "Frank Herbert");
    assert_eq!(b.isbn.0, "978-0441013593");
    assert_eq!(b.year, 1965);
    assert!(b.available, "a new book is available");
}

#[test]
fn describing() {
    let mut b = dune();
    assert_eq!(describe(&b), "Dune by Frank Herbert (1965)");
    b.available = false;
    assert_eq!(describe(&b), "Dune by Frank Herbert (1965) [on loan]");
}

#[test]
fn new_edition_keeps_the_original() {
    let mut first = dune();
    first.available = false;
    let second = second_edition(&first, Isbn(String::from("978-0593099322")), 2019);
    assert_eq!(describe(&second), "Dune by Frank Herbert (2019)", "the new edition is available");
    assert_eq!(second.isbn.0, "978-0593099322");
    assert_eq!(describe(&first), "Dune by Frank Herbert (1965) [on loan]", "the first edition is unchanged");
}
