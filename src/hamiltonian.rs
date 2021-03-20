use crate::WaveFunction;
use crate::Particle;

#[derive(Debug)]
pub struct Hamiltonian {
    lambda: f64,
}

impl Hamiltonian {
    pub fn spherical() -> Self {
        Hamiltonian { lambda: 1. }
    }

    pub fn elliptical(lambda: f64) -> Self {
        Hamiltonian { lambda: lambda }
    }

    fn kinetic<T: WaveFunction>(&self, wf: &T, particles: &Vec<Particle>) -> f64 {
        - 0.5 * wf.laplace(&particles)
    }

    fn trap_potential(&self, particles: &Vec<Particle>) -> f64 {
        let squared_position_sum: f64 = particles.iter().map(|x| x.squared_sum_scaled_z(&self.lambda)).sum();
        0.5 * squared_position_sum
    }

    fn inter_boson_potential(&self, particles: &Vec<Particle>) -> f64 {
        let mut sum: f64 = 0.;
        for i in 0..particles.len() {
            for j in i+1..particles.len() {
                if particles[i].distance_to(&particles[j]) > 0. {
                    sum += 0.;
                } else {
                    sum += f64::INFINITY;
                }
            }
        }
        sum
    }

    pub fn local_energy<T: WaveFunction>(&self, wf: &T, particles: &Vec<Particle>) -> f64 {
        self.kinetic(wf, particles) + self.trap_potential(particles)
    }

    pub fn energy<T: WaveFunction>(&self, wf: &T, particles: &Vec<Particle>) -> f64 {
        self.kinetic(wf, particles) + self.trap_potential(particles) + self.inter_boson_potential(particles)
    }
}
