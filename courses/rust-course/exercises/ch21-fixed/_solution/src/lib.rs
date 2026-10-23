#![allow(non_camel_case_types)]
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Q8_8(pub i16);

impl From<f64> for Q8_8 {
    fn from(n: f64) -> Self {
        Q8_8((n * 256.0).round().clamp(i16::MIN as f64, i16::MAX as f64) as i16)
    }
}

impl From<Q8_8> for f64 {
    fn from(q: Q8_8) -> f64 {
        q.0 as f64 / 256.0
    }
}

impl Add for Q8_8 {
    type Output = Q8_8;
    fn add(self, rhs: Q8_8) -> Q8_8 {
        Q8_8(self.0.saturating_add(rhs.0))
    }
}

impl Sub for Q8_8 {
    type Output = Q8_8;
    fn sub(self, rhs: Q8_8) -> Q8_8 {
        Q8_8(self.0.saturating_sub(rhs.0))
    }
}

impl Mul for Q8_8 {
    type Output = Q8_8;
    fn mul(self, rhs: Q8_8) -> Q8_8 {
        let p = (self.0 as i32 * rhs.0 as i32) >> 8;
        Q8_8(p.clamp(i16::MIN as i32, i16::MAX as i32) as i16)
    }
}

impl Neg for Q8_8 {
    type Output = Q8_8;
    fn neg(self) -> Q8_8 {
        Q8_8(self.0.saturating_neg())
    }
}

impl fmt::Display for Q8_8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.4}", f64::from(*self))
    }
}

pub fn rand_unit(byte: u8) -> f32 {
    let base: u32 = 0b0_01111110_00000000000000000000000;
    let m = f32::from_bits(base | ((byte as u32) << 15));
    2.0 * (m - 0.5)
}

#[cfg(test)]
mod tests;
