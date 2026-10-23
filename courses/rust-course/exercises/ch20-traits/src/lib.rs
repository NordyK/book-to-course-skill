use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn length(self) -> f64 {
        todo!()
    }
}

// TODO: Add, Sub, Mul<f64>, Mul<Vec2> (dot), Neg, AddAssign, Index, IndexMut

pub trait Shape {
    fn name(&self) -> String;
}

pub trait Label {
    fn name(&self) -> String;
}

// TODO: impl Shape and Label for Vec2

pub fn both_names(v: &Vec2) -> (String, String) {
    todo!()
}

#[cfg(test)]
mod tests;
