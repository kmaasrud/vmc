use crate::{WaveFunction, Particle};


#[derive(Clone)]
pub struct Hamiltonian {

}

impl Hamiltonian {

    // --- Kinetic energy ---
    fn kinetic(wf: &WaveFunction, particles: &mut Vec<Particle>) -> f64{
        -0.5 * wf.laplace(particles, false) //??? interacting or not
    }
    // --- Potential energy ---
    fn potential(wf: &WaveFunction, particles: &mut Vec<Particle>) -> f64{
        let sqrd_pos_sum: f64 = particles.iter().map(|x| x.squared_sum_scaled_z(&self.gamma_squared)).sum();
        0.5 * omega.powf(2)*sqrd_pos_sum
    }
   
    fn repulsive(particles: &mut Vec<Particle>)-> f64{
        let sqrd_pos_sum: f64 = particles.iter().map(|x| x.squared_sum()).sum();
        1 / sqrd_pos_sum
    }    

    pub fn hamiltonian(&self, wf: &WaveFunction, particles: &mut Vec<Particle>) -> f64{
        self.kinetic(wf, particles)  + self.potential(wf, particles) + self.repulsive(particles)
    }
   
}
