/// A checked username.
#[derive(Debug, PartialEq)]
pub struct Username(String);

impl Username {
    pub fn parse(s: &str) -> Result<Username, String> {
        let s = s.trim();
        let len = s.chars().count();
        if len < 3 {
            return Err(String::from("too short"));
        }
        if len > 16 {
            return Err(String::from("too long"));
        }
        for c in s.chars() {
            if !(c.is_ascii_alphanumeric() || c == '_') {
                return Err(format!("invalid character: {c}"));
            }
        }
        if s.chars().next().unwrap().is_ascii_digit() {
            return Err(String::from("starts with a digit"));
        }
        Ok(Username(s.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A guess between 1 and 100.
#[derive(Debug)]
pub struct Guess(i32);

impl Guess {
    /// # Panics
    /// If `v` is not between 1 and 100.
    pub fn new(v: i32) -> Guess {
        if !(1..=100).contains(&v) {
            panic!("Guess value must be between 1 and 100, got {v}.");
        }
        Guess(v)
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

#[cfg(test)]
mod tests;
