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

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
    fn name(&self) -> &'static str {
        "circle"
    }
    fn scale(&mut self, k: f64) {
        self.r *= k;
    }
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        self.w * self.h
    }
    fn name(&self) -> &'static str {
        "rect"
    }
    fn scale(&mut self, k: f64) {
        self.w *= k;
        self.h *= k;
    }
}

pub struct Registry {
    shapes: Vec<Box<dyn Shape>>,
}

impl Registry {
    pub fn new() -> Self {
        Registry { shapes: Vec::new() }
    }

    pub fn add(&mut self, s: Box<dyn Shape>) {
        self.shapes.push(s);
    }

    pub fn total_area(&self) -> f64 {
        self.shapes.iter().map(|s| s.area()).sum()
    }

    pub fn largest(&self) -> Option<&dyn Shape> {
        let mut best = self.shapes.first()?;
        for s in &self.shapes {
            if s.area() > best.area() {
                best = s;
            }
        }
        Some(best.as_ref())
    }

    pub fn scale_all(&mut self, k: f64) {
        for s in self.shapes.iter_mut() {
            s.scale(k);
        }
    }

    pub fn count_by_name(&self, name: &str) -> usize {
        self.shapes.iter().filter(|s| s.name() == name).count()
    }
}

pub fn sum_areas<T: Shape>(items: &[T]) -> f64 {
    items.iter().map(|s| s.area()).sum()
}

#[cfg(test)]
mod tests;
