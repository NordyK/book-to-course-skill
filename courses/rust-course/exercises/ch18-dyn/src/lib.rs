use std::fmt::Debug;

pub trait Shape: Debug {
    fn area(&self) -> f64;
    fn name(&self) -> &'static str;
    fn scale(&mut self, k: f64);
}

#[derive(Debug)]
pub struct Circle {
    pub r: f64,
}

#[derive(Debug)]
pub struct Rect {
    pub w: f64,
    pub h: f64,
}

// impl Shape for Circle / Rect

pub struct Registry {
    shapes: Vec<Box<dyn Shape>>,
}

impl Registry {
    pub fn new() -> Self {
        Registry { shapes: Vec::new() }
    }

    pub fn add(&mut self, s: Box<dyn Shape>) {
        todo!()
    }

    pub fn total_area(&self) -> f64 {
        todo!()
    }

    pub fn largest(&self) -> Option<&dyn Shape> {
        todo!()
    }

    pub fn scale_all(&mut self, k: f64) {
        todo!()
    }

    pub fn count_by_name(&self, name: &str) -> usize {
        todo!()
    }
}

pub fn sum_areas<T: Shape>(items: &[T]) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests;
