pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub enum Token {
    Num(i64),
    Op(char),
    Ident(String),
}

pub fn http_class(code: u16) -> &'static str {
    match code {
        100..=199 => "informational",
        200..=299 => "success",
        300..=399 => "redirect",
        400..=499 => "client error",
        500..=599 => "server error",
        _ => "invalid",
    }
}

pub fn quadrant(p: Point) -> String {
    let name = match p {
        Point { x: 0, y: 0 } => "origin",
        Point { y: 0, .. } => "x axis",
        Point { x: 0, .. } => "y axis",
        Point { x, y } if x > 0 && y > 0 => "Q1",
        Point { x, y } if x < 0 && y > 0 => "Q2",
        Point { x, y } if x < 0 && y < 0 => "Q3",
        _ => "Q4",
    };
    name.to_string()
}

pub fn summarize(v: &[i32]) -> String {
    match v {
        [] => String::from("empty"),
        [only] => format!("just {only}"),
        [first, .., last] if first == last => format!("bookends {first}"),
        [first, .., last] => format!("{first}..{last} ({} items)", v.len()),
    }
}

pub fn ticket(age: u32, student: bool) -> u32 {
    match (age, student) {
        (0..=5, _) => 0,
        (6..=17, _) => 5,
        (18..=25, true) => 6,
        (65.., _) => 7,
        _ => 10,
    }
}

pub fn describe(t: Token) -> String {
    match t {
        Token::Num(n @ 0..=9) => format!("digit {n}"),
        Token::Num(n) => format!("number {n}"),
        Token::Op(op @ ('+' | '-' | '*' | '/')) => format!("operator {op}"),
        Token::Op(op) => format!("unknown op {op}"),
        Token::Ident(name) if name.starts_with('_') => format!("private {name}"),
        Token::Ident(name) => format!("name {name}"),
    }
}

#[cfg(test)]
mod tests;
