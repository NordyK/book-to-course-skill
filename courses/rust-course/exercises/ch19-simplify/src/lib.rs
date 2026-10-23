#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(i64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

pub fn simplify(e: Expr) -> Expr {
    todo!()
}

pub fn eval(e: &Expr, x: i64) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests;
