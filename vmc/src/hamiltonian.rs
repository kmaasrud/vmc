use crate::{WaveFunction, Particle};


#[derive(Clone)]
pub struct Hamiltonian {
    kinetic: &f64,
    potential: &f64,
    repulsive: &f64,

}

impl Hamiltonian {

    // --- Kinetic energy ---
    fn kinetic(wf: &WaveFunction, particles: &mut Vec<Particle>) -> f64{
        -0.5 * wf.laplace(particles, false) //??? interacting or not
    }
    // --- Potential energy ---
    fn potential(wf: &WaveFunction, particles: &mut Vec<Particle>) -> f64{
        let omega : f64 = 1.0;
        let sqrd_pos_sum: f64 = particles.iter().map(|x| x.squared_sum()).sum();
        0.5 * omega.powf(2.0)*sqrd_pos_sum
    }
   
    fn repulsive(particles: &mut Vec<Particle>)-> f64{
        let sqrd_pos_sum: f64 = particles.iter().map(|x| x.squared_sum()).sum();
        1.0 / sqrd_pos_sum
    }    

    pub fn energy(&self, wf: &WaveFunction, particles: &mut Vec<Particle>) -> f64{
        self.kinetic  + self.potential + self.repulsive
    }
   
}
