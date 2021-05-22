use crate::{Hermite, Particle, Vector};

#[derive(Clone)]
pub struct WaveFunction {
    pub alpha: f64,
    pub beta: f64,
    pub a: f64,
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
                let mut exp_sum = 0.;
                for (i, particle) in particles.iter().enumerate() {
                    for other in particles[i + 1..].iter() {
                        let fermion_distance: f64 = particle.distance_to(other)?;
                        exp_sum += self.a * fermion_distance / (1. + self.beta * fermion_distance);
                    }
                }

                let r1: f64 = particles[0].squared_sum();
                let r2: f64 = particles[1].squared_sum();

                let result: f64 = c * (-0.5 * self.alpha * omega * (r1 + r2) + exp_sum).exp();

                println!("{}", result);
                Ok(result)
            }
            // This is the general evaluation, using Slater determinants
            _ => Ok(1.),
        }
    }

    // --- Laplacian ---
    /// Returns the Laplacian of the wavefunction evaluated numerically at state of 'particles'.
    /// Returns laplacian for the wavefunction with hermitian polynomials
    pub fn laplace(&self, particles: &mut Vec<Particle>) -> Result<f64, String> {
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

    pub fn laplace_hermitian(&self, particles: &mut Vec<Particle>, nx: usize, ny: usize) -> f64 {
        let r1: f64 = particles[0].squared_sum();
        let r2: f64 = particles[1].squared_sum();
        let omega = 1.0;

        // Hermitian polynomials
        // TODO: The unwraps below should not be left untouched. We should not panic at an error,
        // but rather propagate them up the stack. Will fix ASAP.
        let omega_alpha_sqrt = (omega * self.alpha).sqrt();
        let hnx = Hermite::evaluate(omega_alpha_sqrt * r1.powf(0.5), nx).unwrap();
        let hny = Hermite::evaluate(omega_alpha_sqrt * r2.powf(0.5), ny).unwrap();

        let d_hnx = Hermite::derivative(omega_alpha_sqrt * r1.powf(0.5), nx).unwrap();
        let d_hny = Hermite::derivative(omega_alpha_sqrt * r1.powf(0.5), ny).unwrap();

        let dd_hnx = Hermite::double_derivative(omega_alpha_sqrt * r1.powf(0.5), nx).unwrap();
        let dd_hny = Hermite::double_derivative(omega_alpha_sqrt * r1.powf(0.5), ny).unwrap();

        let r = r1 + r2;
        let omega_alpha = omega * self.alpha;

        let result = (-0.5 * omega_alpha * r).exp()
            * (-2.0 * omega_alpha * r1.powf(0.5) * hny * d_hnx
                - 2.0 * omega_alpha * r2.powf(0.5) * hnx * d_hny
                + omega_alpha * hnx * hny * (omega_alpha * r - 2.0)
                + hny * dd_hnx
                + hnx * dd_hny);
        result
    }

    // --- Gradients ---
    /// Returns the gradient for a particle with regards to the non-interacting part of the
    /// wavefunction
    fn gradient_spf(&self, particle: &Particle) -> Vector {
        let gradient = particle.position.clone();
        match gradient {
            Vector::D1(_) | Vector::D2(_,_) => gradient.scale(-2. * self.alpha),
            Vector::D3(x, y, z) => Vector::D3(-2. * self.alpha * x,
                                                         -2. * self.alpha * y,
                                                         -2. * self.alpha * self.beta * z),
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
            let factor = a / (distance.powi(2) * (distance - a));

            gradient += (particles[i].position - particles[j].position).scale(factor);
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
        (self.gradient_spf(&particles[i]) + self.gradient_interaction(i, particles)).scale(2.)
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
        /* let omega: f64 = 1.;         let c: f64 = 1.; //Defined separately in evaluate() function
        let h: f64 = 0.0001; //Defined separately in laplace() function
        let h2: f64 = h.powi(2); //Defined separately in laplace() function */

        // Spawn a system with defined wavefunction and energy
        let ham: Hamiltonian = Hamiltonian;
        let wf = WaveFunction {
            alpha, beta, a,
        }; // Set beta = gamma
        let mut system: System = System::distributed(2, 2, wf.clone(), ham.clone(), false, 1.);
        system.particles[0].position = Vector::D2(0., 0.); //Just placing the particles at specific positions
        system.particles[1].position = Vector::D2(1., 1.);

        // Define the analytical answer to this problem
        let analytical = 1.; //FILL

        // Assertion
        let tol: f64 = 1E-13;
        // Unwrap is okay here, because a panic means the test fails
        assert!((wf.laplace(&mut system.particles).unwrap() - analytical).abs() < tol);
    }

    #[test]
    fn test_determinism() {
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
        let wf = WaveFunction {
            alpha, beta, a,
        }; // Set beta = gamma
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
}
