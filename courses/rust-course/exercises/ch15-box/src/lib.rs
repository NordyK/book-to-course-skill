#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

pub fn num(v: f64) -> Expr {
    todo!()
}

pub fn add(a: Expr, b: Expr) -> Expr {
    todo!()
}

pub fn mul(a: Expr, b: Expr) -> Expr {
    todo!()
}

pub fn eval(e: &Expr) -> Result<f64, String> {
    todo!()
}

pub fn show(e: &Expr) -> String {
    todo!()
}

pub fn depth(e: &Expr) -> usize {
    todo!()
}

#[cfg(test)]
mod tests;
