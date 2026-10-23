#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

pub fn num(v: f64) -> Expr {
    Expr::Num(v)
}

pub fn add(a: Expr, b: Expr) -> Expr {
    Expr::Add(Box::new(a), Box::new(b))
}

pub fn mul(a: Expr, b: Expr) -> Expr {
    Expr::Mul(Box::new(a), Box::new(b))
}

pub fn eval(e: &Expr) -> Result<f64, String> {
    match e {
        Expr::Num(v) => Ok(*v),
        Expr::Add(a, b) => Ok(eval(a)? + eval(b)?),
        Expr::Mul(a, b) => Ok(eval(a)? * eval(b)?),
        Expr::Div(a, b) => {
            let d = eval(b)?;
            if d == 0.0 {
                return Err(String::from("division by zero"));
            }
            Ok(eval(a)? / d)
        }
        Expr::Neg(a) => Ok(-eval(a)?),
    }
}

pub fn show(e: &Expr) -> String {
    match e {
        Expr::Num(v) => format!("{v}"),
        Expr::Add(a, b) => format!("({} + {})", show(a), show(b)),
        Expr::Mul(a, b) => format!("({} * {})", show(a), show(b)),
        Expr::Div(a, b) => format!("({} / {})", show(a), show(b)),
        Expr::Neg(a) => format!("-({})", show(a)),
    }
}

pub fn depth(e: &Expr) -> usize {
    match e {
        Expr::Num(_) => 1,
        Expr::Add(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) => 1 + depth(a).max(depth(b)),
        Expr::Neg(a) => 1 + depth(a),
    }
}

#[cfg(test)]
mod tests;
