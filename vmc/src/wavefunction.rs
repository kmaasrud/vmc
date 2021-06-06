use crate::{Hermite, Particle, Spin, Vector, a, det};
use nalgebra::{SMatrix, base::dimension::DimMin, Const};

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
    pub jastrow_on: bool,
}

impl WaveFunction {
    //-- Trial wavefunction --
    /// Trial wavefunction for the ground state of the two electron/fermion system.
    /// Returns an f64 representing the wavefunction value
    pub fn evaluate<const N: usize>(&self, particles: &Vec<Particle>) -> Result<f64, String> {
        let c: f64 = 1.0; //normalization constant - dont know value

        match particles.len() {
            // In the case of two particles, evaluating the wavefunction is straight forward.
            2 => {
                let r1: f64 = particles[0].squared_sum();
                let r2: f64 = particles[1].squared_sum();
                let jastrow = if self.jastrow_on { self.evaluate_jastrow(particles) } else { 0. };
                //println!("a: {:.16} || o: {:.16} || r1: {:.16} || r2: {:.16} || jast: {:.16}", self.alpha, self.omega, r1, r2, jastrow);
                Ok(c * (-0.5 * self.alpha * self.omega * (r1 + r2) + jastrow).exp())
            },
            // This is the general evaluation, using Slater determinants
            _ => {
                let slater_matrix: SMatrix<f64, N, N> = self.slater_matrix(particles)?;
                let slater_det = det(Some(&slater_matrix), None).unwrap();
                let jastrow = if self.jastrow_on { self.evaluate_jastrow(particles) } else { 0. };
                Ok(slater_det * jastrow.exp())
            },
        }
    }

    fn evaluate_jastrow(&self, particles: &Vec<Particle>) -> f64 {
        let mut jastrow = 0.;
        let n = particles.len();
        for (i, particle) in particles.iter().enumerate() {
            for (j, other) in particles[i + 1..].iter().enumerate() {
                let distance = particle.distance_to(&other).unwrap();
                jastrow += a(i, j, n) * distance / (1. + self.beta * distance)
            }
        }
        jastrow
    }

    pub fn slater_matrix<const N: usize>(&self, particles: &Vec<Particle>) -> Result<SMatrix<f64, N, N>, String> {
        let n = particles.len();
        let mut slater_matrix: SMatrix<f64, N, N> = SMatrix::repeat(0.);
        for i in 0..n {
            for j in 0..n {
                let nx = crate::QUANTUM_NUMBERS.get(i)
                    .ok_or("System can not have more than 20 particles.")?.0;
                let ny = crate::QUANTUM_NUMBERS.get(i)
                    .ok_or("System can not have more than 20 particles.")?.1;
                slater_matrix[(i, j)] = self.spf(&particles[j], nx, ny).unwrap();
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
    /// Returns the Laplacian of the wavefunction evaluated numerically at state of 'particles'.
    pub fn gradient_numerical<const N: usize>(&self, particles: &Vec<Particle>) -> Result<f64, String> {
        let h: f64 = 0.000001; //stepsize
        let two_h = 2. * h;

        let mut gradient = 0.;
        let mut particles = particles.clone();

        for i in 0..particles.len() {
            for dim in 0..particles[i].dim {
                particles[i].bump_at_dim(dim, 2. * h); // Initial position +h
                let wf_plus = self.evaluate::<N>(&particles)?;

                particles[i].bump_at_dim(dim, -2. * h); // Initial position -h
                let wf_minus = self.evaluate::<N>(&particles)?;

                gradient += (wf_plus - wf_minus) / two_h;

                particles[i].bump_at_dim(dim, h); // Reset back to initial position
            }
        }

        Ok(gradient)
    }
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

    pub fn gradient_slater<const N: usize>(&self, p: usize, particles: &Vec<Particle>, slater_inverse: &SMatrix<f64, N, N>) -> Result<Vector, String> {
        let mut gradient = Particle::new(particles[0].dim).unwrap().position;
        for i in 0..N {
            let nx = QUANTUM_NUMBERS[i].0;
            let ny = QUANTUM_NUMBERS[i].1;
            let d_spf = self.gradient_spf(&particles[p], nx, ny)?;
            gradient += d_spf.scale(slater_inverse[(i, p)]);
        }
        Ok(gradient)
    }

    pub fn gradient_jastrow(&self, p: usize, particles: &Vec<Particle>) -> Result<Vector, String> {
        let mut gradient = Particle::new(particles[0].dim).unwrap().position;
        let n = particles.len();
        for (i, particle) in particles.iter().enumerate() {
            if i == p { continue }
            let distance = particles[p].distance_to(&particle)?;
            let factor = a(p, i, n) / (distance * (1. + self.beta * distance).powi(2));
            gradient += (particles[p].position - particle.position).scale(factor);
        }
        Ok(gradient)
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

                Ok(result) //  - n as f64
            }
        }
    }

    /// Returns the gradient of the wavefunction with regards to beta
    pub fn gradient_beta(&self, particles: &Vec<Particle>) -> Result<f64, String> {
        match particles.len() {
            2 => {
                // Can safely unwrap here, since the particles share dimensionality
                let distance = particles[0].distance_to(&particles[1]).unwrap();
                Ok(- 1. * distance.powi(2) / (1. + self.beta * distance).powi(2))
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
                Ok(result) //- n as f64
            }
        }
    }

    // --- Quantum forces ---
    pub fn quantum_force<const N: usize>(&self, p: usize, particles: &Vec<Particle>, slater_inverse: &SMatrix<f64, N, N>) -> Result<Vector, String> {
        if particles.len() == 2 {
            let a = 1.;
            let r1 = particles[0].position;
            let r2 = particles[1].position;
            let r12 = r1 - r2;
            let r21 = r2 - r1;
            let distance = particles[0].distance_to(&particles[1])?;

            if distance == 0. { return Ok(Vector::D2(0., 0.)) }

            let factor1 = -2. * self.alpha * self.omega;
            let factor2 = 2. * a / (distance * (1. + self.beta * distance));

            Ok(r1.scale(factor1) + r12.scale(factor2) + r2.scale(factor1) + r21.scale(factor2))
        } else {
            let slater_gradient = self.gradient_slater(p, particles, slater_inverse)?;
            let jastrow_gradient = self.gradient_jastrow(p, particles)?;
            Ok((slater_gradient + jastrow_gradient).scale(2.))
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
