use crate::{Hermite, Particle, Vector, System, Spin};
use nalgebra::SMatrix;

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
}

impl WaveFunction {
    //-- Trial wavefunction --
    /// Trial wavefunction for the ground state of the two electron/fermion system.
    /// Returns an f64 representing the wavefunction value
    pub fn evaluate(&self, particles: &Vec<Particle>) -> Result<f64, String> {
        let omega: f64 = 1.0;
        let c: f64 = 1.0; //normalization constant - dont know value

        match particles.len() {
            // In the case of two particles, evaluating the wavefunction is a bit simpler.
            2 => {
                let a = 1.; // TODO: What to do here?
                let mut exp_sum = 0.;
                for (i, particle) in particles.iter().enumerate() {
                    for other in particles[i + 1..].iter() {
                        let fermion_distance: f64 = particle.distance_to(other)?;
                        exp_sum += a * fermion_distance / (1. + self.beta * fermion_distance);
                    }
                }

                let r1: f64 = particles[0].squared_sum();
                let r2: f64 = particles[1].squared_sum();

                let result: f64 = c * (-0.5 * self.alpha * omega * (r1 + r2) + exp_sum).exp();

                println!("{}", result);
                Ok(result)
            }
            // This is the general evaluation, using Slater determinants
            _ => {
                Ok(1.)
            }
        }
    }

    /// Evaluates the single particle wave function  
    pub fn spf(&self, particle: &Particle, nx: usize, ny: usize, omega: f64) -> Result<f64, String> {
        let sqrt_alpha_omega = (self.alpha * omega).sqrt();
        let result = match particle.position {
            Vector::D2(x, y) => {
                Hermite::evaluate(sqrt_alpha_omega * x, nx as usize)?
                    * Hermite::evaluate(sqrt_alpha_omega * y, ny as usize)?
            }
            _ => return Err("spf only supports two dimension right now".to_owned()),
        };

        Ok(result * (-0.5 * self.alpha * omega * particle.position.inner(particle.position)?).exp())
    }

    // --- Laplacian ---
    pub fn laplace(&self, particle_i: usize, particles: &mut Vec<Particle>, interacting: bool) -> f64 {
        let result: f64;
        let n = particles.len();

        for i in 0..(n / 2) {
            for j in 0..(n / 2) {
                let nx = QUANTUM_NUMBERS[particle_i].0;
                let ny = QUANTUM_NUMBERS[particle_i].1;
                result += match QUANTUM_NUMBERS[particle_i].2 {
                    Spin::Up => self.laplace_spf(particles[i], nx, ny) * self.slater_inverse_up[(j, i)],
                    Spin::Down => self.laplace_spf(particles[i + n / 2], nx, ny) * self.slater_inverse_down[(j, i)],
                };
            }
        }
        1.
    }

    /// Returns the Laplacian of the wavefunction evaluated numerically at state of 'particles'.
    /// Returns laplacian for the wavefunction with hermitian polynomials
    pub fn laplace_numerical(&self, particles: &mut Vec<Particle>) -> Result<f64, String> {
        let h: f64 = 0.0001; //stepsize
        let h2 = h.powi(2);

        let mut laplace = 0.;

        let wf = self.evaluate(&particles)?;

        for i in 0..particles.len() {
            for dim in 0..particles[i].dim {
                particles[i].bump_at_dim(dim, h); // Initial position +h
                let wf_plus = self.evaluate(particles)?;

                particles[i].bump_at_dim(dim, -2. * h); // Initial position -h
                let wf_minus = self.evaluate(particles)?;

                particles[i].bump_at_dim(dim, h); // Reset back to initial position

                laplace += (wf_plus - 2. * wf + wf_minus) / h2;
            }
        }
        Ok(laplace / wf)
    }

    /// Returns the Laplacian of the single particle wave function
    /// Works only in two dimensions right now
    pub fn laplace_spf(&self, particle: Particle, nx: usize, ny: usize) -> Result<f64, String> {
        let omega = 1.0;
        let omega_alpha = omega * self.alpha;
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
                    * (-2.0 * omega_alpha * x * hny * d_hnx
                        - 2.0 * omega_alpha * y * hnx * d_hny
                        + omega_alpha * hnx * hny * (omega_alpha * particle.squared_sum() - 2.0)
                        + hny * dd_hnx
                        + hnx * dd_hny))
            },
            _ => return Err("laplace_spf only supports two dimensions right now.".to_owned())
        }

    }

    // --- Gradients ---
    /// Returns the gradient for a particle with regards to the non-interacting part of the
    /// wavefunction
    fn gradient_spf(&self, particle: &Particle) -> Vector {
        let gradient = particle.position.clone();
        match gradient {
            Vector::D1(_) | Vector::D2(_, _) => gradient.scale(-2. * self.alpha),
            Vector::D3(x, y, z) => Vector::D3(
                -2. * self.alpha * x,
                -2. * self.alpha * y,
                -2. * self.alpha * self.beta * z,
            ),
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
            gradient +=   particles[i].position.scale(- self.alpha * self.omega) 
                        + (particles[i].position - particles[j].position).scale(factor)
                        - particles[j].position.scale(self.alpha * self.omega)
                        + (particles[j].position - particles[i].position).scale(factor);
        }

        gradient
    }

    /// Returns the gradient of the wavefunction with regards to alpha
    pub fn gradient_alpha(&self, particles: &Vec<Particle>, nx: usize, ny: usize) -> f64 {
        let r1: f64 = particles[0].squared_sum();
        let r2: f64 = particles[1].squared_sum();
        let omega = 1.0;
         
        // Hermitian polynomials
        // TODO: Find alternative solution to avoid repeated code.
        let omega_alpha_sqrt = (omega * self.alpha).sqrt();
        let hnx = Hermite::evaluate(omega_alpha_sqrt * r1.powf(0.5), nx).unwrap();
        let hny = Hermite::evaluate(omega_alpha_sqrt * r2.powf(0.5), ny).unwrap();

        let _d_alpha_hnx = Hermite::derivative_alpha(nx, r1.powf(0.5), omega, self.alpha);
        let _d_alpha_hny = Hermite::derivative_alpha(ny, r1.powf(0.5), omega, self.alpha);

        let r = r1 + r2;

        let alpha_gradient = (-0.5 * omega * self.alpha * r).exp()
            * (_d_alpha_hnx * hny + hnx * _d_alpha_hny - 0.5 * omega * r * hnx * hny);
        alpha_gradient

        /* let squared_position_sum_sum: f64 = particles.iter().map(|x| x.squared_sum_scaled_z(self.beta)).sum();
        - squared_position_sum_sum */
    }

    // --- Quantum forces ---
    pub fn quantum_force(&self, i: usize, particles: &Vec<Particle>) -> Vector {
        // The gradients need not be devided by the wavefunc since it already has been inside the gradient functions (this cancels terms and easen the computation)
        self.gradient_interaction(i, particles).scale(2.)
    }

    /// Calculates the quantum force of a particle not interacting with its surrounding particles
    pub fn quantum_force_non_interacting(&self, particle: &Particle) -> Vector {
        self.gradient_spf(particle).scale(2.)
    }

    /// Returns the gradient of the wavefunction with regards to x
    pub fn gradient_x(&self, particles: &Vec<Particle>, nx: usize, ny: usize) -> f64 {
        let omega = 1.0;

        let r1: f64 = particles[0].squared_sum();
        let r2: f64 = particles[1].squared_sum();

        //Hermitian polynomials
        // TODO: Find alternative solution to avoid repeated code.
        let omega_alpha_sqrt = (omega * self.alpha).sqrt();
        let hnx = Hermite::evaluate(omega_alpha_sqrt * r1.powf(0.5), nx).unwrap();
        let hny = Hermite::evaluate(omega_alpha_sqrt * r2.powf(0.5), ny).unwrap();

        let d_hnx = Hermite::derivative(omega_alpha_sqrt * r1.powf(0.5), nx).unwrap();

        let gradient: f64 = (-0.5 * omega * self.alpha * (r1 + r2)).exp()
            * hny
            * (d_hnx - hnx * omega * self.alpha * r1.powf(0.5)); //correct ? x*x + y*y = r1 + r2??

        gradient
    }

    /// Returns the gradient of the wavefunction with regards to y
    pub fn gradient_y(&self, particles: &Vec<Particle>, nx: usize, ny: usize) -> f64 {
        let r1: f64 = particles[0].squared_sum();
        let r2: f64 = particles[1].squared_sum();
        let omega = 1.0;

        //Hermitian polynomials
        // TODO: Find alternative solution to avoid repeated code.
        let omega_alpha_sqrt = (omega * self.alpha).sqrt();
        let hnx = Hermite::evaluate(omega_alpha_sqrt * r1.powf(0.5), nx).unwrap();
        let hny = Hermite::evaluate(omega_alpha_sqrt * r2.powf(0.5), ny).unwrap();

        let d_hny = Hermite::derivative(omega_alpha_sqrt * r1.powf(0.5), ny).unwrap();

        let gradient: f64 = (-0.5 * omega * self.alpha * (r1 + r2)).exp()
            * hnx
            * (d_hny - hny * omega * self.alpha * r2.powf(0.5)); //correct ? x*x + y*y = r1 + r2??

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
        let omega: f64 = 1.; //Defined separately in evaluate() function
        let c: f64 = 1.; //Defined separately in evaluate() function
        let h: f64 = 0.0001; //Defined separately in laplace() function
        let h2: f64 = h.powi(2); //Defined separately in laplace() function

        // Spawn a system with defined wavefunction and energy
        let ham: Hamiltonian = Hamiltonian;
        let wf = WaveFunction { alpha, beta, a }; // Set beta = gamma
        let mut system: System = System::distributed(2, 2, wf.clone(), ham.clone(), false, 1.);
        system.particles[0].position = Vector::D2(0., 0.); //Just placing the particles at specific positions
        system.particles[1].position = Vector::D2(1., 1.);

        // Define the analytical answer to this problem
        let r1: f64 = 0.;
        let r2: f64 = (2. as f64).sqrt();
        let r12: f64 = r2;
        let frac: f64 = a / ((1. + beta * r12).powi(2));
        let analytical = 2. * alpha.powi(2) * omega.powi(2) * (r1.powi(2) + r2.powi(2))
            - 4. * alpha * omega
            - frac * 2. * alpha * omega * r12
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
        let omega: f64 = 1.; //Defined separately in evaluate() function
        let c: f64 = 1.; //Defined separately in evaluate() function

        // Spawn a system with defined wavefunction and energy
        let ham: Hamiltonian = Hamiltonian;
        let wf = WaveFunction { alpha, beta, a }; // Set beta = gamma
        let mut system: System = System::distributed(2, 2, wf.clone(), ham.clone(), false, 1.);
        system.particles[0].position = Vector::D2(0., 0.); //Just placing the particles at specific positions
        system.particles[1].position = Vector::D2(1., 1.);

        // Define the analytical answer to this problem
        let analytical = c
            * (-alpha * omega * (0. + 1. * 1. + 1. * 1.) / 2.).exp()
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
        let omega: f64 = 1.; //Defined separately in evaluate() function
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
        let analyticalx = 2. * alpha * omega * 0. + 2. / r12 * frac * 1. - 2. * alpha * omega * 1.
            + 2. / r12 * frac * (-1.);

        let analyticaly = 2. * alpha * omega * 0. + 2. / r12 * frac * 1. - 2. * alpha * omega * 1.
            + 2. / r12 * frac * (-1.);
        let analytical = Vector::D2(analyticalx, analyticaly);

        // Assertion
        let tol: f64 = 1E-13;
        assert!((wf.quantum_force(1, &mut system.particles) - analytical).abs() < tol);
        // Magnus is fixing fancy vectors with subtraction, wait for that
    }
}
