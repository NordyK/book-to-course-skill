use std::fmt;
use std::num::ParseFloatError;

#[derive(Debug, PartialEq)]
pub enum ReadingError {
    Empty,
    NoValue(String),
    BadValue(ParseFloatError),
    OutOfRange(f64),
}

// impl fmt::Display for ReadingError { ... }

// impl From<ParseFloatError> for ReadingError { ... }

/// "name value" -> (name, value), with checks.
pub fn parse_reading(line: &str) -> Result<(String, f64), ReadingError> {
    todo!()
}

/// The mean of all readings, or the first error.
pub fn average(lines: &[&str]) -> Result<f64, ReadingError> {
    todo!()
}

#[cfg(test)]
mod tests;
