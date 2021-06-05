use crate::System;

#[derive(Clone)]
pub struct Hamiltonian;

impl Hamiltonian {
    // --- Kinetic energy ---
    fn kinetic<const N: usize>(sys: &System<N>) -> Result<f64, String> {
        Ok(-0.5 * sys.laplace()?)
    }

    // --- Potential energy ---
    fn potential<const N: usize>(sys: &System<N>) -> f64 {
        // Harmonic oscillator
        let external_potential: f64 = sys.particles.iter().map(|x| x.squared_sum()).sum();

        // Repulsive
        let repulsive = if sys.interacting {
            let mut s = 0.;
            for (i, particle) in sys.particles.iter().enumerate() {
                for other in sys.particles[i + 1..].iter() {
                    // Dimensions should always match, can safely unwrap
                    s += particle.distance_to(other).unwrap();
                }
            }
            1. / s
        } else { 0. };

        0.5 * sys.wf.omega.powf(2.0) * external_potential + repulsive
    }

    /// Calculates the energy of a system of `particles` described by `wf`.
    /// If `non_interacting` is `true`, will calculate the non-interacting energy (unused for now).
    pub fn energy<const N: usize>(sys: &System<N>) -> Result<(f64, f64), String> {
        if N == 200 && !sys.num_laplace && !sys.wf.jastrow_on {
            let distance = sys.particles[0].distance_to(&sys.particles[1])?;
            let r1 = sys.particles[0].squared_sum();
            let r2 = sys.particles[1].squared_sum();
            Ok((2. * sys.wf.alpha * sys.wf.omega + 1. / distance
               + 0.5 * sys.wf.omega.powi(2) * (1. - sys.wf.alpha.powi(2)) * (r1 + r2), 1.))
        } else if N == 200 && !sys.num_laplace {
            let a = 1.; // Hard-coding value of a
            let distance = sys.particles[0].distance_to(&sys.particles[1])?;
            let energy = 2. * sys.wf.alpha * sys.wf.omega + 0.5
                  + sys.wf.omega.powi(2) * (1. - sys.wf.alpha.powi(2)) * (sys.particles[0].squared_sum() + sys.particles[1].squared_sum())
                  - a / (1. + sys.wf.beta * distance).powi(2) * (- sys.wf.alpha * sys.wf.omega * distance
                                                                 + a / (1. + sys.wf.beta * distance).powi(2)
                                                                 + (1. - sys.wf.beta * distance) / (distance * (1. + sys.wf.beta * distance)))
                  + 1. / distance;
            Ok((energy, 1.))
        } else {
            let kinetic = Self::kinetic(sys)?;
            Ok((kinetic + Self::potential(sys), kinetic))
        }
    }
}
