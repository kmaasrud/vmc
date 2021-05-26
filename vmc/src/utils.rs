pub enum SmallVector<T, const N: usize> {
    Inline(usize, [T; N]),
    Dynamic(Vec<T>),
}

impl<T: Copy + Clone, const N: usize> SmallVector<T, N> {
    fn new(v: T, n: usize) -> Self {
        if n <= N {
            Self::Inline(n, [v; N])
        } else {
            Self::Dynamic(vec![v; n])
        }
    }
}

impl<T, const N: usize> SmallVector<T, N> {
    fn iter(&self) -> std::slice::Iter<T> {
        match self {
            Self::Inline(_, array) => array.iter(),
            Self::Dynamic(vec) => vec.iter(),
        }
    }

    fn as_slice(&self) -> &[T] {
        match self {
            Self::Inline(n, array) => &array[0..*n],
            Self::Dynamic(vec) => vec,
        }
    }

    fn as_mut_slice(&mut self) -> &mut [T] {
        match self {
            Self::Inline(n, array) => &mut array[0..*n],
            Self::Dynamic(vec) => vec,
        }
    }
}

use std::ops::{Deref, DerefMut};

impl<T, const N: usize> Deref for SmallVector<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, const N: usize> DerefMut for SmallVector<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

pub fn det(mat: &Vec<Vec<f64>>) -> Option<f64> {
    // Check for non-square matrix
    if mat.len() != mat[0].len() {
        return None;
    };

    // Recursive determinant calculation
    match mat.len() {
        1 => Some(mat[0][0]),
        _ => Some(
            mat[0]
                .iter()
                .enumerate()
                .map(|(i, val)| -1f64.powi(i as i32) * val * det(&sub(mat, i)).unwrap())
                .sum::<f64>(),
        ),
    }
}

fn sub(mat: &Vec<Vec<f64>>, col: usize) -> Vec<Vec<f64>> {
    let mut sub: Vec<Vec<f64>> = mat.clone();

    // Pop first vec item (first row in matrix)
    sub.remove(0);

    // Iterate over rows, remove item at col
    for i in 0..sub.len() {
        sub[i].remove(col);
    }

    sub
}
