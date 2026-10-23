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
    todo!()
}

pub fn quadrant(p: Point) -> String {
    todo!()
}

pub fn summarize(v: &[i32]) -> String {
    todo!()
}

pub fn ticket(age: u32, student: bool) -> u32 {
    todo!()
}

pub fn describe(t: Token) -> String {
    todo!()
}

#[cfg(test)]
mod tests;
