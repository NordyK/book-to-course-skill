use super::*;

#[test]
fn http() {
    assert_eq!(http_class(100), "informational");
    assert_eq!(http_class(204), "success");
    assert_eq!(http_class(301), "redirect");
    assert_eq!(http_class(404), "client error");
    assert_eq!(http_class(599), "server error");
    assert_eq!(http_class(99), "invalid");
    assert_eq!(http_class(600), "invalid");
}

#[test]
fn quadrants() {
    assert_eq!(quadrant(Point { x: 0, y: 0 }), "origin");
    assert_eq!(quadrant(Point { x: 5, y: 0 }), "x axis");
    assert_eq!(quadrant(Point { x: 0, y: -2 }), "y axis");
    assert_eq!(quadrant(Point { x: 1, y: 1 }), "Q1");
    assert_eq!(quadrant(Point { x: -1, y: 1 }), "Q2");
    assert_eq!(quadrant(Point { x: -1, y: -1 }), "Q3");
    assert_eq!(quadrant(Point { x: 1, y: -1 }), "Q4");
}

#[test]
fn summaries() {
    assert_eq!(summarize(&[]), "empty");
    assert_eq!(summarize(&[4]), "just 4");
    assert_eq!(summarize(&[3, 9, 3]), "bookends 3");
    assert_eq!(summarize(&[2, 2]), "bookends 2");
    assert_eq!(summarize(&[1, 5, 8, 2]), "1..2 (4 items)");
}

#[test]
fn tickets() {
    assert_eq!(ticket(3, false), 0);
    assert_eq!(ticket(12, true), 5);
    assert_eq!(ticket(20, true), 6);
    assert_eq!(ticket(20, false), 10);
    assert_eq!(ticket(30, true), 10);
    assert_eq!(ticket(65, false), 7);
    assert_eq!(ticket(80, true), 7);
}

#[test]
fn tokens() {
    assert_eq!(describe(Token::Num(7)), "digit 7");
    assert_eq!(describe(Token::Num(42)), "number 42");
    assert_eq!(describe(Token::Num(-3)), "number -3");
    assert_eq!(describe(Token::Op('*')), "operator *");
    assert_eq!(describe(Token::Op('%')), "unknown op %");
    assert_eq!(describe(Token::Ident(String::from("_tmp"))), "private _tmp");
    assert_eq!(describe(Token::Ident(String::from("count"))), "name count");
}
