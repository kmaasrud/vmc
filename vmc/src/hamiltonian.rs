use crate::{Particle, System};

#[derive(Clone)]
pub struct Hamiltonian;

impl Hamiltonian {
    // --- Kinetic energy ---
    fn kinetic<const N: usize>(sys: &System<N>) -> Result<f64, String> {
        Ok(-0.5 * sys.laplace()?)
    }

    // --- Potential energy ---
    fn potential(omega: f64, particles: &Vec<Particle>, interacting: bool) -> f64 {
        // Harmonic oscillator
        let sqrd_pos_sum: f64 = particles.iter().map(|x| x.squared_sum()).sum();

        // Repulsive
        let distance_sum = if interacting {
            let mut s = 0.;
            for (i, particle) in particles.iter().enumerate() {
                for other in particles[i + 1..].iter() {
                    // Dimensions should always match, can safely unwrap
                    s += particle.distance_to(other).unwrap();
                }
            }
            s
        } else { 1. };

        0.5 * omega.powf(2.0) * sqrd_pos_sum + 1. / distance_sum
    }

    /// Calculates the energy of a system of `particles` described by `wf`.
    /// If `non_interacting` is `true`, will calculate the non-interacting energy (unused for now).
    pub fn energy<const N: usize>(sys: &System<N>) -> Result<f64, String> {
        if sys.particles.len() == 200 {
            let a = 1.; // Hard-coding value of a
            let distance = sys.particles[0].distance_to(&sys.particles[1])?;
            Ok(2. * sys.wf.alpha * sys.wf.omega + 0.5
                  + sys.wf.omega.powi(2) * (1. - sys.wf.alpha.powi(2)) * (sys.particles[0].squared_sum() + sys.particles[1].squared_sum())
                  - a / (1. + sys.wf.beta * distance).powi(2) * (- sys.wf.alpha * sys.wf.omega * distance
                                                                 + a / (1. + sys.wf.beta * distance).powi(2)
                                                                 + (1. - sys.wf.beta * distance) / (distance * (1. + sys.wf.beta * distance)))
                  + 1. / distance)
        } else {
            Ok(Self::kinetic(sys)? + Self::potential(sys.wf.omega, &sys.particles, sys.interacting))
        }
    }
}
