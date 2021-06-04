use nalgebra::{SMatrix, DimSub};

#[derive(Debug)]
pub enum Spin {
    Up,
    Down,
}

pub fn a(i: usize, j: usize, n: usize) -> f64 {
    let n_div_2 = n / 2;
    if (i < n_div_2 && j < n_div_2) || (i >= n_div_2 && j >= n_div_2) {
        1. / 3.
    } else {
        1.
    }
}

// This allocates a lot more than I would've preferred, but when nalgebra refuses to calculate a
// simple determinant, then this'll have to do...
pub fn det<const N: usize>(mat: Option<&SMatrix<f64, N, N>>, vec: Option<&Vec<Vec<f64>>>) -> Option<f64> {
    let mut vec_mat: Vec<Vec<f64>>;
    match mat {
        Some(mat) => {
            vec_mat = vec![vec![0.; N]; N];
            for i in 0..N {
                for j in 0..N {
                    vec_mat[i][j] = mat[(i, j)];
                }
            }
        },
        None => {
            vec_mat = vec.unwrap().clone();
        }
    }

    // Recursive determinant calculation
    match N {
        1 => Some(vec_mat[0][0]),
        _ => Some(vec_mat[0]
                .iter()
                .enumerate()
                .map(|(i, val)| -1f64.powi(i as i32) * val * det::<N>(None, Some(&sub(&vec_mat, i))).unwrap())
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
