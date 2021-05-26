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

pub struct System<const N: usize> {
    pub particles: Vec<Particle>,
    pub dim: usize,
    pub wf: WaveFunction,
    pub ham: Hamiltonian,
    pub slater_matrix_up: SMatrix<f64, N, N>,
    pub slater_matrix_down: SMatrix<f64, N, N>,
    pub slater_inverse_up: SMatrix<f64, N, N>,
    pub slater_inverse_down: SMatrix<f64, N, N>,
    pub interacting: bool,
}

impl<const N: usize> System<N> {
    /// Creates a new system with particles distributed randomly
    pub fn new(
        n_particles: usize,
        dim: usize,
        wf: WaveFunction,
        ham: Hamiltonian,
        interact: bool,
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

        let mut slater_matrix_up: SMatrix<f64, N, N> = SMatrix::repeat(0.);
        let mut slater_matrix_down: SMatrix<f64, N, N> = SMatrix::repeat(0.);

        let n_div_2 = n_particles / 2;
        for i in 0..n_div_2 {
            for j in 0..n_div_2 {
                slater_matrix_up[(i, j)] = wf.spf(particles[i], crate::QUANTUM_NUMBERS[j].0, crate::QUANTUM_NUMBERS[j].1, 1.);
                slater_matrix_down[(i, j)] = wf.spf(particles[i + n_div_2], crate::QUANTUM_NUMBERS[j].0, crate::QUANTUM_NUMBERS[j].1, 1.);
            }
        }

        Ok(System {
            particles: vec![Particle::new(dim)?; n_particles],
            dim,
            wf,
            ham,
            slater_matrix_up,
            slater_matrix_down,
            slater_inverse_up: slater_matrix_up.try_inverse().unwrap(),
            slater_inverse_down: slater_matrix_down.try_inverse().unwrap(),
            interacting: interact,
        })
    }

    /// Change a random particle's position by a random value
    pub fn random_particle_change(&self, step_size: f64) -> Vec<Particle> {
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
        new_particles
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
