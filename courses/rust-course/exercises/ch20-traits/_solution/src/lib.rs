use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn length(self) -> f64 {
        (self * self).sqrt()
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, o: Vec2) -> Vec2 {
        Vec2 { x: self.x + o.x, y: self.y + o.y }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, o: Vec2) -> Vec2 {
        Vec2 { x: self.x - o.x, y: self.y - o.y }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, k: f64) -> Vec2 {
        Vec2 { x: self.x * k, y: self.y * k }
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = f64;
    fn mul(self, o: Vec2) -> f64 {
        self.x * o.x + self.y * o.y
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2 { x: -self.x, y: -self.y }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, o: Vec2) {
        self.x += o.x;
        self.y += o.y;
    }
}

impl Index<usize> for Vec2 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index {i} out of range for Vec2"),
        }
    }
}

impl IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("index {i} out of range for Vec2"),
        }
    }
}

pub trait Shape {
    fn name(&self) -> String;
}

pub trait Label {
    fn name(&self) -> String;
}

impl Shape for Vec2 {
    fn name(&self) -> String {
        String::from("vector")
    }
}

impl Label for Vec2 {
    fn name(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

pub fn both_names(v: &Vec2) -> (String, String) {
    (Shape::name(v), Label::name(v))
}

#[cfg(test)]
mod tests;
