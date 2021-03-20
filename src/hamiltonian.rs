use crate::WaveFunction;
use crate::Particle;

#[derive(Debug)]
pub struct Hamiltonian {
    lambda: f64,
    omega: f64,
}

impl Hamiltonian {
    pub fn spherical(omega: f64) -> Self {
        Hamiltonian { lambda: 1., omega: omega }
    }

    pub fn elliptical(lambda: f64, omega: f64) -> Self {
        Hamiltonian { lambda: lambda, omega: omega }
    }

    fn kinetic<T: WaveFunction>(&self, wf: &T, particles: &Vec<Particle>) -> f64 {
        - self.omega / 2. * wf.laplace(&particles)
    }

    fn trap_potential(&self, particles: &Vec<Particle>) -> f64 {
        let squared_position_sum: f64 = particles.iter().map(|x| x.squared_sum_scaled_z(&self.lambda)).sum();
        self.omega / 2. * squared_position_sum
    }

    fn inter_boson_potential(&self, particles: &Vec<Particle>) -> f64 {
        1.
    }
}
