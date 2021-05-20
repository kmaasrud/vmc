use std::ops::{Add, Sub, AddAssign};
use self::Vector::*;

#[derive(Debug, Clone)]
pub enum Vector {
    D1(f64),
    D2(f64, f64),
    D3(f64, f64, f64)
}

impl Vector {
    pub fn scale(&self, factor: f64) -> Self {
        match self {
            D1(x) => D1(factor * x),
            D2(x, y) => D2(factor * x, factor * y),
            D3(x, y, z) => D3(factor * x, factor * y, factor * z),
        }
    }

    pub fn inner(&self, other: Self) -> f64 {
        match (self, other) {
            (D1(x1), D1(x2)) => x1 * x2,
            (D2(x1, y1), D2(x2, y2)) => x1 * x2 + y1 * y2,
            (D3(x1, y1, z1), D3(x2, y2, z2)) => x1 * x2 + y1 * y2 + z1 * z2
        }  
    }
}

impl Add for Vector {
    type Output = Self;

    /// Adds a vector to another.
    /// If the dimensions do not match, the function will return self.
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (D1(x1), D1(x2)) => D1(x1 + x2),
            (D2(x1, y1), D2(x2, y2)) => D2(x1 + x2, y1 + y2),
            (D3(x1, y1, z1), D3(x2, y2, z2)) => D3(x1 + x2, y1 + y2, z1 + z2),
            _ => self
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    /// Subtracts a vector from another.
    /// If the dimensions do not match, the function will return self.
    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (D1(x1), D1(x2)) => D1(x1 - x2),
            (D2(x1, y1), D2(x2, y2)) => D2(x1 - x2, y1 - y2),
            (D3(x1, y1, z1), D3(x2, y2, z2)) => D3(x1 - x2, y1 - y2, z1 - z2),
            _ => self
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
