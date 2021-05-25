use self::Vector::*;
use std::ops::{Add, AddAssign, Sub};

/// Vector in 1, 2 or 3 dimensions containing `f64`'s.
#[derive(Debug, Clone, Copy)]
pub enum Vector {
    D1(f64),
    D2(f64, f64),
    D3(f64, f64, f64),
}

impl Vector {
    pub fn scale(&self, factor: f64) -> Self {
        match self {
            D1(x) => D1(factor * x),
            D2(x, y) => D2(factor * x, factor * y),
            D3(x, y, z) => D3(factor * x, factor * y, factor * z),
        }
    }

    pub fn inner(&self, other: Self) -> Result<f64, String> {
        match (*self, other) {
            (D1(x1), D1(x2)) => Ok(x1 * x2),
            (D2(x1, y1), D2(x2, y2)) => Ok(x1 * x2 + y1 * y2),
            (D3(x1, y1, z1), D3(x2, y2, z2)) => Ok(x1 * x2 + y1 * y2 + z1 * z2),
            _ => Err("Incompatible dimensions.".to_owned()),
        }
    }

    pub fn get(&self, index: usize) -> Option<f64> {
        match self {
            D1(x) => {
                if index != 0 {
                    return None;
                } else {
                    Some(*x)
                }
            }
            D2(x, y) => match [*x, *y].get(index) {
                Some(val) => Some(*val),
                None => None,
            },
            D3(x, y, z) => match [*x, *y, *z].get(index) {
                Some(val) => Some(*val),
                None => None,
            },
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
            _ => self,
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
            _ => self,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}
