use crate::{montecarlo::SampledValues, Hamiltonian, Particle, System};
use nalgebra::SMatrix;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use std::collections::HashMap;

/// Trait for Metropolis samplers.
pub trait Metropolis {
    fn new(step_size: f64) -> Self;
    fn step<const N: usize>(
        &mut self,
        sys: &mut System<N>,
    ) -> Result<Option<SampledValues>, String>;

    fn hastings_check(acceptance_factor: f64) -> bool {
        if acceptance_factor >= 1. {
            true
        } else {
            let mut rng = thread_rng();
            let uniform = Uniform::new(0., 1.);
            uniform.sample(&mut rng) < acceptance_factor
        }
    }

    fn sample<const N: usize>(sys: &mut System<N>) -> Result<SampledValues, String> {
        let d_wf_deriv = sys.wf.gradient_alpha(&sys.particles, 0, 0);
        // The 1.0 inputted below is a placeholder for the omega value. We are testing over
        // different omega values. TODO: Consider storing omega in the System struct instead of
        // passing it through the stack.
        let d_energy = Hamiltonian::energy(&sys)?;

        let mut map = HashMap::new();
        map.insert("energy".to_string(), d_energy);
        map.insert("energy_sqrd".to_string(), d_energy.powi(2));
        map.insert("wf_deriv".to_string(), d_wf_deriv);
        map.insert("wf_deriv_times_energy".to_string(), d_wf_deriv * d_energy);
        Ok(SampledValues { map })
    }

    fn greens(x: &Particle, y: &Particle) -> Result<f64, String> {
        let vec_sum = x.position - y.position - y.qforce.scale(0.0025);
        // Ignoring denominator of Greens since it cancels later
        Ok((-vec_sum.inner(vec_sum)? / 0.01).exp())
    }
}

pub struct BruteForceMetropolis {
    step_size: f64,
}

impl Metropolis for BruteForceMetropolis {
    fn new(step_size: f64) -> Self {
        Self { step_size }
    }

    fn step<const N: usize>(
        &mut self,
        sys: &mut System<N>,
    ) -> Result<Option<SampledValues>, String> {
        let (new_particles, p) = sys.random_particle_change(self.step_size);
        let mut new_inverse: SMatrix<f64, N, N> = SMatrix::repeat(0.);

        let acceptance_factor = match N {
            2 => {
                let wf_old = sys.wf.evaluate(&sys.particles, sys.interacting)?;
                let wf_new = sys.wf.evaluate(&new_particles, sys.interacting)?;
                wf_new.powi(2) / wf_old.powi(2)
            }
            _ => {
                new_inverse = sys.next_slater_inverse(&new_particles, p)?;
                sys.next_slater_ratio(p, &new_inverse)
            }
        };

        if Self::hastings_check(acceptance_factor) {
            sys.particles = new_particles;
            sys.slater_inverse = new_inverse;
            sys.slater_ratio = acceptance_factor;
            Ok(Some(Self::sample(sys)?))
        } else {
            Ok(None)
        }
    }
}

pub struct ImportanceMetropolis;

impl Metropolis for ImportanceMetropolis {
    fn new(_: f64) -> Self {
        Self
    }

    fn step<const N: usize>(
        &mut self,
        sys: &mut System<N>,
    ) -> Result<Option<SampledValues>, String> {
        let mut new_inverse: SMatrix<f64, N, N> = SMatrix::repeat(0.);

        // Make a step
        let (new_particles, p) = sys.quantum_force_particle_change()?;

        // Calculate the acceptance factor
        let greens_factor = Self::greens(&sys.particles[p], &new_particles[p])?
            / Self::greens(&new_particles[p], &sys.particles[p])?;
        let acceptance_factor = match N {
            2 => {
                let wf_old = sys.wf.evaluate(&sys.particles, sys.interacting)?;
                let wf_new = sys.wf.evaluate(&new_particles, sys.interacting)?;
                greens_factor * wf_new.powi(2) / wf_old.powi(2)
            }
            _ => {
                new_inverse = sys.next_slater_inverse(&new_particles, p)?;
                greens_factor * sys.next_slater_ratio(p, &new_inverse)
            }
        };

        if Self::hastings_check(acceptance_factor) {
            sys.particles = new_particles;
            sys.slater_inverse = new_inverse;
            sys.slater_ratio = acceptance_factor;
            Ok(Some(Self::sample(sys)?))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Hamiltonian, System, Vector, WaveFunction};

    #[test]
    fn test_hastings_check() {
        assert!(BruteForceMetropolis::hastings_check(1.)); //Panics if it returns false
        assert!(BruteForceMetropolis::hastings_check(2.));
        assert!(!BruteForceMetropolis::hastings_check(0.)) //Panics if it returns true
    }

    #[test]
    fn test_greens() {
        // Special greens variables
        let diffusion_coeff: f64 = 0.5;
        let dt: f64 = 0.005;

        let p0 = Particle::from_vector(Vector::D2(0., 0.));
        let pold = Particle::from_vector(Vector::D2(0.01, 0.01));
        let pnew = Particle::from_vector(Vector::D2(0.011, 0.011));
        pnew.qforce += Vector::D2(0.2, 0.2);

        // a = 1, alpha = 0.5 omega = 1, beta = 1
        let r12: f64 = p0.distance_to(&pold).unwrap();
        let qforce_vec = p0.position.scale(-2. * 0.5)
            + (p0.position - pold.position).scale(2. / (r12 * (1. + r12).powf(2.)))
            + pold.position.scale(-2. * 0.5)
            + (pold.position - p0.position).scale(2. / (r12 * (1. + r12).powf(2.)));
        println!("{:?}", qforce_vec);
        let relpos = pnew.position - pold.position;
        let poldscaled = pold.position.scale(diffusion_coeff * dt);
        let therest = (relpos - poldscaled)
            .scale(-1.)
            .scale(1. / (4. * diffusion_coeff * dt)); //Fuck me for doing this ugly shit
        let analytical: f64 = therest.inner(therest).unwrap();

        //Assertation
        let tol: f64 = 1E-13;
        assert!((BruteForceMetropolis::greens(&pnew, &pold).unwrap() - analytical) < tol);
    }

    #[test]
    fn test_sample() {
        let tol = 0.000000000000001;
        // Spawn a system with defined wavefunction and energy
        let ham: Hamiltonian = Hamiltonian;
        let wf = WaveFunction {
            alpha: 0.5,
            beta: 1.,
            a: 1.,
        }; // Set beta = gamma
        let mut system: System = System::distributed(10, 3, wf, ham.clone(), false, 1.).unwrap();

        // Get SampledValues object containing the map from sample func
        let smpldvls = BruteForceMetropolis::sample(&mut system).unwrap();

        //Generate own energies and wf deriv
        let d_energy = system.ham.energy(&system.wf, &mut system.particles, 1.0);
        let d_wf_deriv = system.wf.gradient_alpha(&system.particles, 0, 0); //Set n in hermite polynomials to 0 this sould be changed
        assert!(
            (system
                .ham
                .energy(&system.wf, &mut system.particles, 1.0)
                .unwrap()
                - system
                    .ham
                    .energy(&system.wf, &mut system.particles, 1.0)
                    .unwrap())
            .abs()
                < tol
        );
        // Assertion
        //assert_eq!(smpldvls.map["energy"], d_energy);
        //assert_eq!(smpldvls.map["energy_sqrd"], d_energy.powi(2));
        assert!((smpldvls.map["wf_deriv"] - d_wf_deriv).abs() < tol);
        //assert_eq!(smpldvls.map["wf_deriv_times_energy"], d_wf_deriv * d_energy);
    }
}
