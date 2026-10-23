use std::fmt;
use std::num::ParseFloatError;

#[derive(Debug, PartialEq)]
pub enum ReadingError {
    Empty,
    NoValue(String),
    BadValue(ParseFloatError),
    OutOfRange(f64),
}

impl fmt::Display for ReadingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReadingError::Empty => write!(f, "empty line"),
            ReadingError::NoValue(name) => write!(f, "no value for {name}"),
            ReadingError::BadValue(e) => write!(f, "bad value: {e}"),
            ReadingError::OutOfRange(v) => write!(f, "out of range: {v}"),
        }
    }
}

impl std::error::Error for ReadingError {}

impl From<ParseFloatError> for ReadingError {
    fn from(e: ParseFloatError) -> Self {
        ReadingError::BadValue(e)
    }
}

/// "name value" -> (name, value), with checks.
pub fn parse_reading(line: &str) -> Result<(String, f64), ReadingError> {
    let line = line.trim();
    if line.is_empty() {
        return Err(ReadingError::Empty);
    }
    let Some((name, value)) = line.split_once(' ') else {
        return Err(ReadingError::NoValue(line.to_string()));
    };
    let v: f64 = value.trim().parse()?;
    if !(-50.0..=150.0).contains(&v) {
        return Err(ReadingError::OutOfRange(v));
    }
    Ok((name.to_string(), v))
}

/// The mean of all readings, or the first error.
pub fn average(lines: &[&str]) -> Result<f64, ReadingError> {
    if lines.is_empty() {
        return Err(ReadingError::Empty);
    }
    let mut sum = 0.0;
    for line in lines {
        let (_, v) = parse_reading(line)?;
        sum += v;
    }
    Ok(sum / lines.len() as f64)
}

#[cfg(test)]
mod tests;
