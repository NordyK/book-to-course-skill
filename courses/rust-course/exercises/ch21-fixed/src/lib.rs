#![allow(non_camel_case_types)]
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Q8_8(pub i16);

impl From<f64> for Q8_8 {
    fn from(n: f64) -> Self {
        todo!()
    }
}

impl From<Q8_8> for f64 {
    fn from(q: Q8_8) -> f64 {
        todo!()
    }
}

// TODO: Add, Sub, Mul, Neg, Display

pub fn rand_unit(byte: u8) -> f32 {
    todo!()
}

#[cfg(test)]
mod tests;
