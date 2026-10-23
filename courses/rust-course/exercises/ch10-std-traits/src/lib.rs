use std::fmt;
use std::ops::{Add, AddAssign, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Money {
    pub cents: i64,
}

// impl fmt::Display for Money

// impl From<i64> for Money

// impl TryFrom<&str> for Money  (type Error = String)

// impl Add, Sub, Neg, AddAssign for Money

// impl std::iter::Sum for Money

#[cfg(test)]
mod tests;
