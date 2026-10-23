#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(i64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

pub fn simplify(e: Expr) -> Expr {
    match e {
        Expr::Add(a, b) => {
            let (a, b) = (simplify(*a), simplify(*b));
            match (&a, &b) {
                (Expr::Num(x), Expr::Num(y)) => Expr::Num(x + y),
                (_, Expr::Num(0)) => a,
                (Expr::Num(0), _) => b,
                _ => Expr::Add(Box::new(a), Box::new(b)),
            }
        }
        Expr::Mul(a, b) => {
            let (a, b) = (simplify(*a), simplify(*b));
            match (&a, &b) {
                (Expr::Num(x), Expr::Num(y)) => Expr::Num(x * y),
                (_, Expr::Num(0)) | (Expr::Num(0), _) => Expr::Num(0),
                (_, Expr::Num(1)) => a,
                (Expr::Num(1), _) => b,
                _ => Expr::Mul(Box::new(a), Box::new(b)),
            }
        }
        Expr::Neg(inner) => match simplify(*inner) {
            Expr::Num(n) => Expr::Num(-n),
            Expr::Neg(x) => *x,
            other => Expr::Neg(Box::new(other)),
        },
        leaf => leaf,
    }
}

pub fn eval(e: &Expr, x: i64) -> i64 {
    match e {
        Expr::Num(n) => *n,
        Expr::Var(_) => x,
        Expr::Add(a, b) => eval(a, x) + eval(b, x),
        Expr::Mul(a, b) => eval(a, x) * eval(b, x),
        Expr::Neg(a) => -eval(a, x),
    }
}

#[cfg(test)]
mod tests;
