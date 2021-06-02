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
