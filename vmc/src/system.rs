use crate::{
    Hamiltonian,
    Particle,
    Vector,
    WaveFunction,
};

use rand::distributions::{Distribution, Uniform};
use rand::{prelude::random, thread_rng};
use rand_distr::Normal;
use nalgebra::base::SMatrix;
use nalgebra::base::SVector;
use nalgebra::base::MatrixSlice;

pub struct System<const N: usize> {
    pub particles: Vec<Particle>,
    pub dim: usize,
    pub omega: f64,
    pub wf: WaveFunction,
    pub ham: Hamiltonian,
    interacting: bool,
    slater_matrix: SMatrix<f64, N, N>,
    slater_inverse: SMatrix<f64, N, N>,
    slater_ratio: f64,
    v: SVector<f64, N>,
}

impl<const N: usize> System<N> {
    /// Creates a new system with particles distributed randomly
    pub fn new(
        n_particles: usize,
        dim: usize,
        wf: WaveFunction,
        ham: Hamiltonian,
        interact: bool,
        omega: f64,
        spread: f64,
    ) -> Result<Self, String> {
        let mut rng = thread_rng();
        let uniform = Uniform::new(0., 1.);
        let mut particles = vec![Particle::new(dim)?; n_particles];

        for i in 0..particles.len() {
            // Make a new randomly placed particle
            let new_particle = Particle::from_vector(match dim {
                1 => Vector::D1(uniform.sample(&mut rng) - 0.5),
                2 => Vector::D2(
                    uniform.sample(&mut rng) - 0.5,
                    uniform.sample(&mut rng) - 0.5,
                ),
                _ => Vector::D3(
                    uniform.sample(&mut rng) - 0.5,
                    uniform.sample(&mut rng) - 0.5,
                    uniform.sample(&mut rng) - 0.5,
                ),
            });

            particles[i].position = new_particle.position.scale(spread);
        }

        let mut slater_matrix: SMatrix<f64, N, N> = SMatrix::repeat(0.);

        for i in 0..n_particles {
            for j in 0..n_particles {
                slater_matrix[(i, j)] = wf.spf(&particles[i], crate::QUANTUM_NUMBERS[j].0, crate::QUANTUM_NUMBERS[j].1, 1.).unwrap();
            }
        }

        Ok(System {
            particles: vec![Particle::new(dim)?; n_particles],
            dim,
            wf,
            ham,
            omega,
            interacting: interact,
            slater_matrix,
            slater_inverse: slater_matrix.try_inverse().unwrap(),
            slater_ratio: 0.,
            v: SVector::<f64, N>::repeat(0.),
        })
    }


    // NOTE: Storing the Laplacian here is messy, but it allows a much cleaner function signature.
    // WaveFunction and System are intimately tied together, and should've ideally been made as one
    // struct, but it is too late for that now.
    /// Returns the Laplacian at this current state
    pub fn laplace(&self) -> Result<f64, String> {
        let result: f64;
        let n = self.particles.len();

        for i in 0..n {
            for j in 0..n {
                let nx = crate::QUANTUM_NUMBERS[j].0;
                let ny = crate::QUANTUM_NUMBERS[j].1;
                result += self.wf.laplace_spf(self.particles[i], nx, ny)? * self.slater_inverse[(j, i)];
            }
        }

        Ok(result)
    }

    pub fn next_slater_inverse(&mut self, new_particles: &Vec<Particle>, p: usize) -> Result<SMatrix<f64, N, N>, String> {
        // Find v_p
        for i in 0..N {
            let ny = crate::QUANTUM_NUMBERS[i].1;
            let nx = crate::QUANTUM_NUMBERS[i].0;
            self.v[i] = self.wf.spf(&new_particles[p], nx, ny, self.omega)? - self.wf.spf(&self.particles[p], nx, ny, self.omega)?
        }

        let mut new_inverse: SMatrix<f64, N, N> = SMatrix::repeat(0.);
        let identity: SMatrix<f64, N, N> = SMatrix::identity();
        // NOTE: Double for-loop, so the complexity is O(n^2), contary to what we mention in Method...
        for i in 0..N {
            for j in 0..N {
                new_inverse[(i, j)] = (identity[(i, j)] - self.slater_inverse[(i, p)] * self.v[i] / self.slater_ratio) * self.slater_inverse[(i, j)];
            }
        }

        Ok(new_inverse)
    }

    pub fn next_slater_ratio(&self, p: usize, new_inverse: &SMatrix<f64, N, N>) -> f64 {
        let mut result = 0.;
        let u = new_inverse.column(p);
        for i in 0..N {
            result += self.v[i] * u[i];
        }
        1. + result
    }

    /// Change a random particle's position by a random value
    pub fn random_particle_change(&self, step_size: f64) -> (Vec<Particle>, usize) {
        let mut new_particles = self.particles.clone();
        let i = random::<usize>() % self.particles.len();
        let add = match new_particles[i].position {
            Vector::D1(_) => Vector::D1(random::<f64>() - 0.5),
            Vector::D2(_, _) => Vector::D2(random::<f64>() - 0.5, random::<f64>() - 0.5),
            Vector::D3(_, _, _) => Vector::D3(
                random::<f64>() - 0.5,
                random::<f64>() - 0.5,
                random::<f64>() - 0.5,
            ),
        };
        new_particles[i].position += add.scale(step_size);
        (new_particles, i)
    }

    /// Takes in a step size and returns the next particle state of the system.
    pub fn quantum_force_particle_change(&mut self) -> (Vec<Particle>, usize) {
        let mut rng = thread_rng();
        let normal = Normal::new(0., 1.).unwrap();

        // 0.005 is hard-coded solution for delta t in Langevin equation
        let qf_step_size: f64 = 0.005;

        // Picks one random particle to do the change for
        let i = random::<usize>() % self.particles.len();

        self.particles[i].qforce = if self.interacting {
            self.wf.quantum_force(i, &self.particles)
        } else {
            self.wf.quantum_force_non_interacting(&self.particles[i])
        };

        // Clones the last particle state of the system
        let mut new_particles = self.particles.clone();

        // Do Langevin equation (NOTE: Consider making a function for random vectors to avoid this mess)
        new_particles[i].position = new_particles[i].position
            + self.particles[i].qforce.scale(0.5 * qf_step_size)
            + (match new_particles[i].position {
                Vector::D1(_) => Vector::D1(normal.sample(&mut rng)),
                Vector::D2(_, _) => Vector::D2(normal.sample(&mut rng), normal.sample(&mut rng)),
                Vector::D3(_, _, _) => Vector::D3(
                    normal.sample(&mut rng),
                    normal.sample(&mut rng),
                    normal.sample(&mut rng),
                ),
            })
            .scale(qf_step_size.sqrt());

        // Calculate quantum force of new state
        new_particles[i].qforce = if self.interacting {
            self.wf.quantum_force(i, &new_particles)
        } else {
            self.wf.quantum_force_non_interacting(&new_particles[i])
        };

        (new_particles, i)
    }
}
