pub fn longest_word<'a>(text: &'a str) -> &'a str {
    todo!()
}

pub fn longer_of<'a>(a: &'a str, b: &'a str) -> &'a str {
    todo!()
}

pub fn after_prefix<'a>(text: &'a str, prefix: &str) -> Option<&'a str> {
    todo!()
}

pub struct Tokens<'a> {
    rest: &'a str,
}

impl<'a> Tokens<'a> {
    pub fn new(text: &'a str) -> Self {
        todo!()
    }

    pub fn next_token(&mut self) -> Option<&'a str> {
        todo!()
    }
}

#[cfg(test)]
mod tests;
