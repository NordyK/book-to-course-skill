use super::*;

fn sample() -> Expr {
    add(num(1.0), mul(num(2.0), num(3.0)))
}

#[test]
fn evaluates() {
    assert_eq!(eval(&sample()), Ok(7.0));
    assert_eq!(eval(&Expr::Neg(Box::new(num(4.0)))), Ok(-4.0));
    assert_eq!(eval(&Expr::Div(Box::new(num(1.0)), Box::new(num(4.0)))), Ok(0.25));
}

#[test]
fn division_by_zero() {
    let e = add(num(1.0), Expr::Div(Box::new(num(1.0)), Box::new(add(num(2.0), num(-2.0)))));
    assert_eq!(eval(&e), Err(String::from("division by zero")));
}

#[test]
fn shows() {
    assert_eq!(show(&sample()), "(1 + (2 * 3))");
    assert_eq!(show(&Expr::Neg(Box::new(num(4.5)))), "-(4.5)");
    assert_eq!(show(&Expr::Div(Box::new(num(1.0)), Box::new(num(2.0)))), "(1 / 2)");
}

#[test]
fn depths() {
    assert_eq!(depth(&num(1.0)), 1);
    assert_eq!(depth(&sample()), 3);
}

#[test]
fn helpers_box() {
    assert_eq!(add(num(1.0), num(2.0)), Expr::Add(Box::new(Expr::Num(1.0)), Box::new(Expr::Num(2.0))));
}
