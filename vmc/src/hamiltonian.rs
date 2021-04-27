use crate::{WaveFunction, Particle};


#[derive(Clone)]
pub struct Hamiltonian;

impl Hamiltonian {
    // --- Kinetic energy ---
    fn kinetic(wf: &WaveFunction, particles: &mut Vec<Particle>) -> f64{
        -0.5 * wf.laplace(particles, false) //??? interacting or not
    }

    // --- Potential energy ---
    fn potential(particles: &Vec<Particle>) -> f64 {
        let omega : f64 = 1.0;
        let sqrd_pos_sum: f64 = particles.iter().map(|x| x.squared_sum()).sum();
        0.5 * omega.powf(2.0) * sqrd_pos_sum
    }
   
    // --- Repulsive energy --- (This is just another potential, right? TODO: Possibility for
    // combining with Hamiltonian::potential?)
    fn repulsive(particles: &mut Vec<Particle>)-> f64{
        let sqrd_pos_sum: f64 = particles.iter().map(|x| x.squared_sum()).sum();
        1.0 / sqrd_pos_sum
    }

    /// Calculates the energy of a system of `particles` described by `wf`.
    /// If `non_interacting` is `true`, will calculate the non-interacting energy (unused for now).
    pub fn energy(&self, wf: &WaveFunction, particles: &mut Vec<Particle>, _non_interacting: bool) -> f64{
        Self::kinetic(wf, particles) + Self::potential(particles) + Self::repulsive(particles)
    }
   
}
