use super::*;

fn num(n: i64) -> Expr {
    Expr::Num(n)
}
fn var(s: &str) -> Expr {
    Expr::Var(s.to_string())
}
fn add(a: Expr, b: Expr) -> Expr {
    Expr::Add(Box::new(a), Box::new(b))
}
fn mul(a: Expr, b: Expr) -> Expr {
    Expr::Mul(Box::new(a), Box::new(b))
}
fn neg(a: Expr) -> Expr {
    Expr::Neg(Box::new(a))
}

#[test]
fn constants_fold() {
    assert_eq!(simplify(add(num(2), num(3))), num(5));
    assert_eq!(simplify(mul(add(num(1), num(2)), num(4))), num(12));
    assert_eq!(simplify(neg(num(5))), num(-5));
}

#[test]
fn identities() {
    assert_eq!(simplify(add(var("x"), num(0))), var("x"));
    assert_eq!(simplify(add(num(0), var("x"))), var("x"));
    assert_eq!(simplify(mul(var("x"), num(1))), var("x"));
    assert_eq!(simplify(mul(num(1), var("x"))), var("x"));
    assert_eq!(simplify(mul(var("x"), num(0))), num(0));
    assert_eq!(simplify(mul(num(0), add(var("x"), var("y")))), num(0));
}

#[test]
fn nested() {
    assert_eq!(simplify(add(mul(var("x"), num(1)), mul(num(2), num(3)))), add(var("x"), num(6)));
    assert_eq!(simplify(neg(neg(add(var("y"), num(0))))), var("y"));
    assert_eq!(simplify(add(var("x"), mul(num(0), var("z")))), var("x"));
    assert_eq!(simplify(mul(var("x"), var("y"))), mul(var("x"), var("y")));
}

#[test]
fn evaluation() {
    let e = add(mul(var("x"), var("x")), num(1));
    assert_eq!(eval(&e, 4), 17);
    assert_eq!(eval(&neg(e.clone()), 2), -5);
    assert_eq!(eval(&simplify(e.clone()), 3), eval(&e, 3));
}
