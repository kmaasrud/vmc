use crate::{Particle, WaveFunction};

#[derive(Clone)]
pub struct Hamiltonian;

impl Hamiltonian {
    // --- Kinetic energy ---
    fn kinetic(wf: &WaveFunction, particles: &mut Vec<Particle>) -> Result<f64, String> {
        match wf.laplace(particles) {
            Ok(laplace) => Ok(-0.5 * laplace),
            Err(e) => Err(e),
        }
    }

    // --- Potential energy ---
    fn potential(omega: f64, particles: &Vec<Particle>) -> f64 {
        let sqrd_pos_sum: f64 = particles.iter().map(|x| x.squared_sum()).sum();
        0.5 * omega.powf(2.0) * sqrd_pos_sum
    }

    // --- Repulsive energy ---
    pub fn repulsive(particles: &mut Vec<Particle>) -> f64 {
        let mut distance_sum: f64 = 0.;
        for (i, particle) in particles.iter().enumerate() {
            for other in particles[i + 1..].iter() {
                // Dimensions should always match, can safely unwrap
                distance_sum += particle.distance_to(other).unwrap();
            }
        }
        1.0 / distance_sum
    }

    /// Calculates the energy of a system of `particles` described by `wf`.
    /// If `non_interacting` is `true`, will calculate the non-interacting energy (unused for now).
    pub fn energy(&self, wf: &WaveFunction, particles: &mut Vec<Particle>, omega: f64) -> Result<f64, String> {
        Ok(Self::kinetic(wf, particles)?
            + Self::potential(omega, particles)
            + Self::repulsive(particles))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vector;

    #[test]
    /// Test repulsive energy for two particles with a distance of 2 between
    fn test_repulsive() {
        let want: f64 = 0.5;
        let got: f64 = Hamiltonian::repulsive(&mut vec![
            Particle::from_vector(Vector::D3(0., 0., 0.)),
            Particle::from_vector(Vector::D3(2., 0., 0.)),
        ]);

        assert_eq!(want, got);
    }
}
