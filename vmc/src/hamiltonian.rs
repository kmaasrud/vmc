use crate::{Particle, System};

#[derive(Clone)]
pub struct Hamiltonian;

impl Hamiltonian {
    // --- Kinetic energy ---
    fn kinetic<const N: usize>(sys: &System<N>) -> Result<f64, String> {
        Ok(-0.5 * sys.laplace()?)
    }

    // --- Potential energy ---
    fn potential(omega: f64, particles: &Vec<Particle>) -> f64 {
        let sqrd_pos_sum: f64 = particles.iter().map(|x| x.squared_sum()).sum();
        0.5 * omega.powf(2.0) * sqrd_pos_sum
    }

    // --- Repulsive energy ---
    pub fn repulsive(particles: &Vec<Particle>) -> f64 {
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
    pub fn energy<const N: usize>(sys: &System<N>) -> Result<f64, String> {
        if sys.particles.len() == 200 {
            let a = 1. / 3.;
            let distance = sys.particles[0].distance_to(&sys.particles[1])?;
            Ok(2. * sys.wf.alpha * sys.wf.omega + 0.5
                  + sys.wf.omega.powi(2) * (1. - sys.wf.alpha.powi(2)) * (sys.particles[0].squared_sum() + sys.particles[1].squared_sum())
                  - a / (1. + sys.wf.beta * distance).powi(2) * (- sys.wf.alpha * sys.wf.omega * distance
                                                                 + a / (1. + sys.wf.beta * distance).powi(2)
                                                                 + (1. - sys.wf.beta * distance) / (distance * (1. + sys.wf.beta * distance)))
                  + 1. / distance)
        } else {
            Ok(Self::kinetic(sys)? + Self::potential(sys.wf.omega, &sys.particles))
        }
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
