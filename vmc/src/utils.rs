pub fn det(mat: &Vec<Vec<f64>>) -> Option<f64> {
    println!("{:?}", mat);
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

fn main() {
    let mat = vec![vec![3., 2., 8.], vec![10., 1., 7.], vec![1., 2., 5.]];
    println!("{}", det(&mat).unwrap());
}
