pub struct Hermite;

const C: [f64; 36] = [
    1., 0., 2., -2., 0., 4., 0., -12., 0., 8., 12., 0., -48., 0., 16., 0., 120., 0., -160., 0.,
    32., -120., 0., 720., 0., -480., 0., 64., 0., -1680., 0., 3360., 0., -1344., 0., 128.,
];

impl Hermite {
    /// Evaluates the Hermite polynomial of order n.
    /// Supports only orders 0-7, the first four are hard-coded for efficiency.
    pub fn evaluate(x: f64, n: usize) -> Result<f64, String> {
        if n > 7 {
            return Err("This function does not support orders higher than 7.".to_owned());
        }

        let result = match n {
            0 => 1.,
            1 => 2. * x,
            2 => 4. * x.powi(2) - 2.,
            3 => 8. * x.powi(3) - 12. * x,
            _ => {
                let m = (1..=n).sum::<usize>();
                (0..=n).map(|i| C[i + m] * x.powi(i as i32)).sum()
            }
        };

        Ok(result)
    }

    /// Evaluates the derivative of the Hermite polynomial of order n.
    /// Supports only orders 0-7, the first four are hard-coded for efficiency.
    pub fn derivative(x: f64, n: usize) -> Result<f64, String> {
        let result = match n {
            0 => 0.,
            1 => 2.,
            2 => 8. * x,
            3 => 24. * x.powi(2) - 12.0,
            _ => {
                // Use a two point approximation of the derivative for efficiency
                let h = 0.00000001;
                (Self::evaluate(x + h, n)? - Self::evaluate(x - h, n)?) / (2. * h)
            }
        };

        Ok(result)
    }

    /// Evaluates the second derivative of the Hermite polynomial of order n.
    /// Supports only orders 0-7, the first four are hard-coded for efficiency.
    pub fn double_derivative(x: f64, n: usize) -> Result<f64, String> {
        let result = match n {
            0 => 0.,
            1 => 0.,
            2 => 8.,
            3 => 48. * x,
            _ => {
                // Use a three point approximation of the double derivative for efficiency
                let h = 0.00000001;
                (Self::evaluate(x + h, n)? - 2. * Self::evaluate(x, n)? + Self::evaluate(x - h, n)?)
                    / h.powi(2)
            }
        };

        Ok(result)
    }

    // TODO: Find a way of generalizing the variable scaling we do on our x values, to alleviate
    // the need for lots of specialized functions in addition to this one.
    pub fn derivative_alpha(n: usize, x: f64, omega: f64, alpha: f64) -> f64 {
        let sqrt_omega_alpha: f64 = (omega * alpha).powf(0.5);
        match n {
            1 => x * (omega / alpha).powf(0.5),
            2 => 4.0 * x * x * omega,
            3 => 12.0 * x * x * x * omega * sqrt_omega_alpha - 6.0 * (omega / alpha).powf(0.5),
            _ => 0.0, // println!("not valid n") //should prob write something else here
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let tol = 0.000000000000001;
        let want = -1.64;
        let got = Hermite::evaluate(0.3, 2).expect("Hermite::evalute returned an error.");
        assert!((want - got).abs() < tol);

        let want = -2.37618e7;
        let got = Hermite::evaluate(-15., 5).expect("Hermite::evalute returned an error.");
        assert_eq!(want, got);
        assert!((want - got).abs() < tol);
    }

    #[test]
    fn test_returns_error() {
        match Hermite::evaluate(1., 8) {
            Ok(_) => {
                panic!("Hermite::evalute did not error correctly when the order was too high.")
            }
            Err(_) => {}
        }
    }
}
