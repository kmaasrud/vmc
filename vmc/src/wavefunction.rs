use crate::{Hermite, Particle, Spin, System, Vector, a};
use nalgebra::{SMatrix, linalg::LU};

// Hard-coding quantum states of up to 20 particles. This is done for speed, an should be
// generalized if this code is to be used seriously
pub const QUANTUM_NUMBERS: [(usize, usize, Spin); 20] = [
    (0, 0, Spin::Up),
    (0, 0, Spin::Down),
    (1, 0, Spin::Up),
    (1, 0, Spin::Down),
    (0, 1, Spin::Up),
    (0, 1, Spin::Down),
    (2, 0, Spin::Up),
    (2, 0, Spin::Down),
    (1, 1, Spin::Up),
    (1, 1, Spin::Down),
    (0, 2, Spin::Up),
    (0, 2, Spin::Down),
    (3, 0, Spin::Up),
    (3, 0, Spin::Down),
    (2, 1, Spin::Up),
    (2, 1, Spin::Down),
    (1, 2, Spin::Up),
    (1, 2, Spin::Down),
    (0, 3, Spin::Up),
    (0, 3, Spin::Down),
];

#[derive(Clone)]
pub struct WaveFunction {
    pub alpha: f64,
    pub beta: f64,
    pub omega: f64,
}

impl WaveFunction {
    //-- Trial wavefunction --
    /// Trial wavefunction for the ground state of the two electron/fermion system.
    /// Returns an f64 representing the wavefunction value
    pub fn evaluate<const N: usize>(&self, particles: &Vec<Particle>) -> Result<f64, String> {
        let c: f64 = 1.0; //normalization constant - dont know value

        match particles.len() {
            // In the case of two particles, evaluating the wavefunction is a bit simpler.
            2 => {
                let mut exp_sum = 0.;
                for (i, particle) in particles.iter().enumerate() {
                    for (j, other) in particles[i + 1..].iter().enumerate() {
                        let fermion_distance: f64 = particle.distance_to(other)?;
                        exp_sum += a(i, j, 2) * fermion_distance / (1. + self.beta * fermion_distance);
                    }
                }

                let r1: f64 = particles[0].squared_sum();
                let r2: f64 = particles[1].squared_sum();

                let result: f64 = c * (-0.5 * self.alpha * self.omega * (r1 + r2) + exp_sum).exp();

                Ok(result)
            }
            // This is the general evaluation, using Slater determinants
            n => {
                // TODO: Of course creating the determinant has to be a hassle... Looking at this tomorrow
                /* let slater_matrix: SMatrix<f64, N, N> = self.slater_matrix(particles)?;
                let slater_det = slater_matrix.determinant() */
                let slater_det = 1.;
                let mut jastrow = 1.;
                for (i, particle) in particles.iter().enumerate() {
                    for (j, other) in particles[i + 1..].iter().enumerate() {
                        let distance = particle.distance_to(&other).unwrap();
                        jastrow += a(i, j, n) * distance / (1. + self.beta * distance)
                    }
                }
                Ok(slater_det * jastrow.exp())
            },
        }
    }

    pub fn slater_matrix<const N: usize>(&self, particles: &Vec<Particle>) -> Result<SMatrix<f64, N, N>, String> {
        let n = particles.len();
        let mut slater_matrix: SMatrix<f64, N, N> = SMatrix::repeat(0.);
        for i in 0..n {
            for j in 0..n {
                let nx = crate::QUANTUM_NUMBERS.get(j)
                    .ok_or("System can not have more than 20 particles.")?.0;
                let ny = crate::QUANTUM_NUMBERS.get(j)
                    .ok_or("System can not have more than 20 particles.")?.1;
                slater_matrix[(i, j)] = self.spf(&particles[i], nx, ny).unwrap();
            }
        }
        Ok(slater_matrix)
    }

    /// Evaluates the single particle wave function  
    pub fn spf(&self, particle: &Particle, nx: usize, ny: usize) -> Result<f64, String> {
        let sqrt_alpha_omega = (self.alpha * self.omega).sqrt();
        let result = match particle.position {
            Vector::D2(x, y) => {
                Hermite::evaluate(sqrt_alpha_omega * x, nx)?
                    * Hermite::evaluate(sqrt_alpha_omega * y, ny)?
            }
            _ => return Err("spf only supports two dimension right now".to_owned()),
        };

        Ok(result * (-0.5 * self.alpha * self.omega * particle.position.inner(particle.position)?).exp())
    }

    // --- Laplacian ---
    /// Returns the Laplacian of the wavefunction evaluated numerically at state of 'particles'.
    /// Returns laplacian for the wavefunction with hermitian polynomials
    pub fn laplace_numerical<const N: usize>(
        &self,
        particles: &Vec<Particle>,
    ) -> Result<f64, String> {
        let h: f64 = 0.000001; //stepsize
        let h2 = h.powi(2);

        let mut laplace = 0.;
        let mut particles = particles.clone();

        let wf = self.evaluate::<N>(&particles)?;

        for i in 0..particles.len() {
            for dim in 0..particles[i].dim {
                particles[i].bump_at_dim(dim, h); // Initial position +h
                let wf_plus = self.evaluate::<N>(&particles)?;

                particles[i].bump_at_dim(dim, -2. * h); // Initial position -h
                let wf_minus = self.evaluate::<N>(&particles)?;

                particles[i].bump_at_dim(dim, h); // Reset back to initial position

                laplace += (wf_plus - 2. * wf + wf_minus) / h2;
            }
        }

        Ok(laplace)
    }

    /// Returns the Laplacian of the single particle wave function
    /// Works only in two dimensions right now
    pub fn laplace_spf(&self, particle: &Particle, nx: usize, ny: usize) -> Result<f64, String> {
        let omega_alpha = self.omega * self.alpha;
        let omega_alpha_sqrt = omega_alpha.sqrt();

        match particle.position {
            Vector::D2(x, y) => {
                let hnx = Hermite::evaluate(omega_alpha_sqrt * x, nx)?;
                let hny = Hermite::evaluate(omega_alpha_sqrt * y, ny)?;
                let d_hnx = Hermite::derivative(omega_alpha_sqrt * x, nx)? * omega_alpha_sqrt;
                let d_hny = Hermite::derivative(omega_alpha_sqrt * y, ny)? * omega_alpha_sqrt;
                let dd_hnx = Hermite::double_derivative(omega_alpha_sqrt * x, nx)? * omega_alpha_sqrt;
                let dd_hny = Hermite::double_derivative(omega_alpha_sqrt * y, ny)? * omega_alpha_sqrt;

                Ok((-0.5 * omega_alpha * particle.squared_sum()).exp()
                    * (-2.0 * omega_alpha * x * hny * d_hnx - 2.0 * omega_alpha * y * hnx * d_hny
                        + omega_alpha * hnx * hny * (omega_alpha * particle.squared_sum() - 2.0)
                        + hny * dd_hnx
                        + hnx * dd_hny))
            }
            _ => return Err("laplace_spf only supports two dimensions right now.".to_owned()),
        }
    }

    // --- Gradients ---
    /// Returns the gradient for a particle with regards to the non-interacting part of the
    /// wavefunction
    fn gradient_spf(&self, particle: &Particle, nx: usize, ny: usize) -> Result<Vector, String> {
        let gradient = particle.position.clone();
        let omega_alpha = self.omega * self.alpha;
        let omega_alpha_sqrt = omega_alpha.sqrt();
        match gradient {
            Vector::D2(x, y) => {
                let hnx = Hermite::evaluate(omega_alpha_sqrt * x, nx)?;
                let hny = Hermite::evaluate(omega_alpha_sqrt * y, ny)?;
                let d_hnx = Hermite::derivative(omega_alpha_sqrt * x, nx)? * omega_alpha_sqrt;
                let d_hny = Hermite::derivative(omega_alpha_sqrt * y, ny)? * omega_alpha_sqrt;
                Ok(Vector::D2(
                    hny * (d_hnx - hnx * omega_alpha * x),
                    hnx * (d_hny - hny * omega_alpha * y),
                ).scale((-0.5 * omega_alpha * particle.squared_sum()).exp()))
            }
            _ => return Err("gradient_spf only supports two dimensions right now.".to_owned()),
        }
    }

    /// Returns the gradient for a particle with regards to the interaction-part of the
    /// wavefunction
    fn gradient_interaction(&self, i: usize, particles: &Vec<Particle>) -> Vector {
        // Can safely unwrap this. particles[0] has a valid dimensionality, so this will always work
        let mut gradient = Particle::new(particles[0].dim).unwrap().position;
        let a: f64 = 0.0043;

        for j in 0..particles.len() {
            if i == j {
                continue;
            }

            // Can safely unwrap distance_to. The dimensions are guaranteed to be equal
            let distance: f64 = particles[i].distance_to(&particles[j]).unwrap();
            let factor = a / (distance * (1. + self.beta * distance).powi(2));

            // This actually calculates the derivative devided by the actual wavefunction, so not dPsi itself, but rather dPsi / Psi.
            gradient += particles[i].position.scale(-self.alpha * self.omega)
                + (particles[i].position - particles[j].position).scale(factor)
                - particles[j].position.scale(self.alpha * self.omega)
                + (particles[j].position - particles[i].position).scale(factor);
        }

        gradient
    }

    /// Returns the gradient of the wavefunction with regards to alpha
    pub fn gradient_alpha(&self, particles: &Vec<Particle>) -> Result<f64, String> {
        match particles.len() {
            2 => {
                Ok(-0.5 * self.omega * (particles[0].squared_sum() + particles[1].squared_sum()))
            },
            _ => {
                let mut result = 0.;
                let factor = 0.5 * (self.omega / self.alpha).sqrt();
                let omega_alpha_sqrt = (self.omega * self.alpha).sqrt();

                let n = particles.len();
                for i in 0..n {
                    for j in 0..n {
                        let nx = QUANTUM_NUMBERS[j].0;
                        let ny = QUANTUM_NUMBERS[j].1;
                        let (x, y) = match particles[i].position {
                            Vector::D2(x, y) => (x, y),
                            _ => return Err("gradient_alpha supports only two dimensions for now.".to_owned())
                        };
                        let hnx = Hermite::evaluate(omega_alpha_sqrt * x, nx).unwrap();
                        let hny = Hermite::evaluate(omega_alpha_sqrt * y, ny).unwrap();
                        let d_alpha_hnx = Hermite::derivative_alpha(nx, x, self.omega, self.alpha)?;
                        let d_alpha_hny = Hermite::derivative_alpha(ny, y, self.omega, self.alpha)?;
                        result += factor * x * d_alpha_hnx / hnx + factor * y * d_alpha_hny / hny;
                    }
                }

                Ok(result - n as f64)
            }
        }
    }

    pub fn gradient_beta(&self, particles: &Vec<Particle>) -> f64 {
        match particles.len() {
            2 => {
                // Can safely unwrap here, since the particles share dimensionality
                let distance = particles[0].distance_to(&particles[1]).unwrap();
                - 1. * distance.powi(2) / (1. + self.beta * distance).powi(2)
            },
            _ => {
                let mut result = 0.;
                let n = particles.len();
                for i in 0..n {
                    for j in 0..n {
                        if i == j { continue }
                        // Can safely unwrap here, since the particles share dimensionality
                        let distance = particles[i].distance_to(&particles[j]).unwrap();
                        result -= a(i, j, n) * distance.powi(2) / (1. + self.beta * distance).powi(2)
                    }
                }
                result
            }
        }
    }

    // --- Quantum forces ---
    pub fn quantum_force(&self, i: usize, particles: &Vec<Particle>) -> Result<Vector, String> {
        if particles.len() == 2 {
            let a = 1.;
            let r1 = particles[0].position;
            let r2 = particles[1].position;
            let r12 = r1 - r2;
            let r21 = r2 - r1;
            let distance = particles[0].distance_to(&particles[1])?;

            let factor1 = -2. * self.alpha * self.omega;
            let factor2 = 2. * a / (distance * (1. + self.beta * distance));

            Ok(r1.scale(factor1) + r12.scale(factor2) + r2.scale(factor1) + r21.scale(factor2))
        } else {
            // The gradients need not be devided by the wavefunc since it already has been inside the gradient functions (this cancels terms and easen the computation)
            Ok(self.gradient_interaction(i, particles).scale(2.))
        }
    }

    /// Calculates the quantum force of a particle not interacting with its surrounding particles
    pub fn quantum_force_non_interacting(&self, particle: &Particle, nx: usize, ny: usize) -> Result<Vector, String> {
        Ok(self.gradient_spf(particle, nx, ny)?.scale(2.))
    }

    /// Returns the gradient of the wavefunction with regards to x
    pub fn gradient_x(&self, particles: &Vec<Particle>, nx: usize, ny: usize) -> f64 {
        let r1: f64 = particles[0].squared_sum();
        let r2: f64 = particles[1].squared_sum();

        //Hermitian polynomials
        // TODO: Find alternative solution to avoid repeated code.
        let omega_alpha_sqrt = (self.omega * self.alpha).sqrt();
        let hnx = Hermite::evaluate(omega_alpha_sqrt * r1.powf(0.5), nx).unwrap();
        let hny = Hermite::evaluate(omega_alpha_sqrt * r2.powf(0.5), ny).unwrap();

        let d_hnx = Hermite::derivative(omega_alpha_sqrt * r1.powf(0.5), nx).unwrap();

        let gradient: f64 = (-0.5 * self.omega * self.alpha * (r1 + r2)).exp()
            * hny
            * (d_hnx - hnx * self.omega * self.alpha * r1.powf(0.5)); //correct ? x*x + y*y = r1 + r2??

        gradient
    }

    /// Returns the gradient of the wavefunction with regards to y
    pub fn gradient_y(&self, particles: &Vec<Particle>, nx: usize, ny: usize) -> f64 {
        let r1: f64 = particles[0].squared_sum();
        let r2: f64 = particles[1].squared_sum();

        //Hermitian polynomials
        // TODO: Find alternative solution to avoid repeated code.
        let omega_alpha_sqrt = (self.omega * self.alpha).sqrt();
        let hnx = Hermite::evaluate(omega_alpha_sqrt * r1.powf(0.5), nx).unwrap();
        let hny = Hermite::evaluate(omega_alpha_sqrt * r2.powf(0.5), ny).unwrap();

        let d_hny = Hermite::derivative(omega_alpha_sqrt * r1.powf(0.5), ny).unwrap();

        let gradient: f64 = (-0.5 * self.omega * self.alpha * (r1 + r2)).exp()
            * hnx
            * (d_hny - hny * self.omega * self.alpha * r2.powf(0.5)); //correct ? x*x + y*y = r1 + r2??

        gradient
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Hamiltonian, System};

    #[test]
    fn test_laplace() {
        // System parameters
        let alpha: f64 = 0.5;
        let beta: f64 = 1.;
        let a: f64 = 1.;

        // The below is defined separately in evaluate() function
        let c: f64 = 1.; //Defined separately in evaluate() function
        let h: f64 = 0.0001; //Defined separately in laplace() function
        let h2: f64 = h.powi(2); //Defined separately in laplace() function

        // Spawn a system with defined wavefunction and energy
        let wf = WaveFunction { alpha, beta, a }; // Set beta = gamma
        let mut system: System = System::new(2, 2, wf.clone(), false, 1.);
        system.particles[0].position = Vector::D2(0., 0.); //Just placing the particles at specific positions
        system.particles[1].position = Vector::D2(1., 1.);

        // Define the analytical answer to this problem
        let r1: f64 = 0.;
        let r2: f64 = (2. as f64).sqrt();
        let r12: f64 = r2;
        let frac: f64 = a / ((1. + beta * r12).powi(2));
        let analytical = 2. * alpha.powi(2) * self.omega.powi(2) * (r1.powi(2) + r2.powi(2))
            - 4. * alpha * self.omega
            - frac * 2. * alpha * self.omega * r12
            + 2. * frac * (frac + 1. / r12 - 2. * beta / (1. + beta * r12));

        // Assertion
        let tol: f64 = 1E-13;
        // Unwrap is okay here, because a panic means the test fails
        assert!((wf.laplace(&mut system.particles).unwrap() - analytical).abs() < tol);
    }

    #[test]
    fn test_evaluate_determinism() {
        // Spawn a system with defined wavefunction and energy
        let ham: Hamiltonian = Hamiltonian;
        let wf = WaveFunction {
            alpha: 0.5,
            beta: 1.,
            a: 1.,
        }; // Set beta = gamma
        let system: System = System::distributed(10, 3, wf.clone(), ham.clone(), false, 1.);

        // Is it deterministic?
        assert_eq!(
            wf.evaluate(&system.particles),
            wf.evaluate(&system.particles)
        );
    }

    #[test]
    fn test_evaluate_against_analytical() {
        // System parameters
        let alpha: f64 = 0.5;
        let beta: f64 = 1.;
        let a: f64 = 1.;
        let c: f64 = 1.; //Defined separately in evaluate() function

        // Spawn a system with defined wavefunction and energy
        let ham: Hamiltonian = Hamiltonian;
        let wf = WaveFunction { alpha, beta, a }; // Set beta = gamma
        let mut system: System = System::distributed(2, 2, wf.clone(), ham.clone(), false, 1.);
        system.particles[0].position = Vector::D2(0., 0.); //Just placing the particles at specific positions
        system.particles[1].position = Vector::D2(1., 1.);

        // Define the analytical answer to this problem
        let analytical = c
            * (-alpha * self.omega * (0. + 1. * 1. + 1. * 1.) / 2.).exp()
            * (a * ((1. * 1. + 1. * 1.) as f64).sqrt()
                / (1. + beta * ((1. * 1. + 1. * 1.) as f64).sqrt()))
            .exp();

        // Assertion
        let tol: f64 = 1E-13;
        // Unwrap is okay here, because a panic means the test fails
        assert!((wf.evaluate(&system.particles).unwrap() - analytical).abs() < tol);
    }

    #[test]
    fn test_quantum_force() {
        // System parameters
        let alpha: f64 = 0.5;
        let beta: f64 = 1.;
        let a: f64 = 1.;

        // The below is defined separately in evaluate() function
        let c: f64 = 1.; //Defined separately in evaluate() function
        let h: f64 = 0.0001; //Defined separately in laplace() function
        let h2: f64 = h.powi(2); //Defined separately in laplace() function

        // Spawn a system with defined wavefunction and energy
        let ham: Hamiltonian = Hamiltonian;
        let wf = WaveFunction { alpha, beta, a }; // Set beta = gamma
        let mut system: System = System::distributed(2, 2, wf.clone(), ham.clone(), false, 1.);
        system.particles[0].position = vec![0., 0.]; //Just placing the particles at specific positions
        system.particles[1].position = vec![1., 1.];

        // Define the analytical answer to this problem
        let r1: f64 = 0.;
        let r2: f64 = (2. as f64).sqrt();
        let r12: f64 = r2;
        let r21: f64 = -r2;
        let frac: f64 = a / ((1. + beta * r12).powi(2));
        let analyticalx = 2. * alpha * self.omega * 0. + 2. / r12 * frac * 1.
            - 2. * alpha * self.omega * 1.
            + 2. / r12 * frac * (-1.);

        let analyticaly = 2. * alpha * self.omega * 0. + 2. / r12 * frac * 1.
            - 2. * alpha * self.omega * 1.
            + 2. / r12 * frac * (-1.);
        let analytical = Vector::D2(analyticalx, analyticaly);

        // Assertion
        let tol: f64 = 1E-13;
        assert!((wf.quantum_force(1, &mut system.particles) - analytical).abs() < tol);
        // Magnus is fixing fancy vectors with subtraction, wait for that
    }
}
