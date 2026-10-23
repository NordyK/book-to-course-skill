use super::*;

fn m(c: i64) -> Money {
    Money { cents: c }
}

#[test]
fn display() {
    assert_eq!(m(1230).to_string(), "$12.30");
    assert_eq!(m(-5).to_string(), "-$0.05");
    assert_eq!(Money::default().to_string(), "$0.00");
}

#[test]
fn from_dollars() {
    assert_eq!(Money::from(3), m(300));
    let x: Money = 2.into();
    assert_eq!(x, m(200), "From gives Into");
}

#[test]
fn parsing() {
    assert_eq!(Money::try_from("12.30"), Ok(m(1230)));
    assert_eq!(Money::try_from("-0.05"), Ok(m(-5)));
    assert_eq!(Money::try_from("7"), Ok(m(700)));
    assert_eq!(Money::try_from("1.5"), Err(String::from("bad amount: 1.5")));
    assert_eq!(Money::try_from("abc"), Err(String::from("bad amount: abc")));
    assert_eq!(Money::try_from("1.x5"), Err(String::from("bad amount: 1.x5")));
}

#[test]
fn arithmetic() {
    assert_eq!(Money::from(3) + m(45), m(345));
    assert_eq!(m(100) - m(250), m(-150));
    assert_eq!(-m(5), m(-5));
    let mut wallet = m(10);
    wallet += m(5);
    assert_eq!(wallet, m(15));
}

#[test]
fn summing_and_ordering() {
    let items = vec![m(199), m(1), m(300)];
    let total: Money = items.iter().copied().sum();
    assert_eq!(total, m(500));
    assert_eq!(items.iter().copied().max(), Some(m(300)), "derived Ord");
}
