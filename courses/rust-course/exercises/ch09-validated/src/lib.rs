/// A checked username.
#[derive(Debug, PartialEq)]
pub struct Username(String);

impl Username {
    pub fn parse(s: &str) -> Result<Username, String> {
        todo!()
    }

    pub fn as_str(&self) -> &str {
        todo!()
    }
}

/// A guess between 1 and 100.
#[derive(Debug)]
pub struct Guess(i32);

impl Guess {
    /// # Panics
    /// If `v` is not between 1 and 100.
    pub fn new(v: i32) -> Guess {
        todo!()
    }

    pub fn value(&self) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests;
