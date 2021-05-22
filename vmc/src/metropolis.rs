use crate::{montecarlo::SampledValues, Particle, System};
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use std::collections::HashMap;

/// Trait for Metropolis samplers.
pub trait Metropolis {
    fn new(step_size: f64) -> Self;
    fn step(&mut self, sys: &mut System) -> Result<Option<SampledValues>, String>;

    fn hastings_check(acceptance_factor: f64) -> bool {
        if acceptance_factor >= 1. {
            true
        } else {
            let mut rng = thread_rng();
            let uniform = Uniform::new(0., 1.);
            uniform.sample(&mut rng) < acceptance_factor
        }
    }

    fn sample(sys: &mut System) -> Result<SampledValues, String> {
        let d_wf_deriv = sys.wf.gradient_alpha(&sys.particles, 0, 0);
        // The 1.0 inputted below is a placeholder for the omega value. We are testing over
        // different omega values. TODO: Consider storing omega in the System struct instead of
        // passing it through the stack.
        let d_energy = sys.ham.energy(&sys.wf, &mut sys.particles, 1.0)?;

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

    fn step(&mut self, sys: &mut System) -> Result<Option<SampledValues>, String> {
        // Make a step
        let next_step = sys.random_particle_change(self.step_size);

        // Evaluate wavefunction for old and new states
        let wf_old: f64 = sys.wf.evaluate(&sys.particles)?;
        let wf_new: f64 = sys.wf.evaluate(&next_step)?;

        if Self::hastings_check(wf_new.powi(2) / wf_old.powi(2)) {
            sys.particles = next_step;
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

    fn step(&mut self, sys: &mut System) -> Result<Option<SampledValues>, String> {
        // Make a step
        let (next_step, i) = sys.quantum_force_particle_change();

        // Evaluate wavefunction for old and new states
        let wf_old: f64 = sys.wf.evaluate(&sys.particles)?;
        let wf_new: f64 = sys.wf.evaluate(&next_step)?;

        // Calculate the acceptance factor
        let greens_factor = Self::greens(&sys.particles[i], &next_step[i])?
            / Self::greens(&next_step[i], &sys.particles[i])?;
        let acceptance_factor = greens_factor * wf_new.powi(2) / wf_old.powi(2);

        if Self::hastings_check(acceptance_factor) {
            sys.particles = next_step;
            Ok(Some(Self::sample(sys)?))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Hamiltonian, System, WaveFunction};

    #[test]
    fn test_hastings_check() {
        assert!(BruteForceMetropolis::hastings_check(1.)); //Panics if it returns false
        assert!(BruteForceMetropolis::hastings_check(2.));
        assert!(!BruteForceMetropolis::hastings_check(0.)) //Panics if it returns true
    }

    #[test]
    fn test_greens() {
        // System parameters
        let alpha: f64 = 0.5;
        let beta: f64 = 1.;
        let a: f64 = 1.;

        // The below is defined separately in evaluate() function
        let omega: f64 = 1.;        //Defined separately in evaluate() function
        let c: f64 = 1.;            //Defined separately in evaluate() function
        let h: f64 = 0.0001;        //Defined separately in laplace() function
        let h2: f64 = h.powi(2);    //Defined separately in laplace() function

        // Spawn a system with defined wavefunction and energy
        let ham: Hamiltonian = Hamiltonian;
        let wf = WaveFunction {
            alpha, beta, a,
        }; // Set beta = gamma
        let mut system: System = System::distributed(2, 2, wf.clone(), ham.clone(), false, 1.);
        system.particles[0].position = vec![0., 0.]; //Just placing the particles at specific positions
        system.particles[1].position = vec![1., 1.];
        // Make next step used in greens
        let (next_step, i) = system.quantum_force_particle_change();
        // Name the old and new particle for eazy comparison
        let oldpos = system.particles[i];
        let newpos = next_step[i];

        // Special greens variables
        let diffusion_coeff: f64 = 0.5;
        let dt: f64 = 0.005;

        
        let xdim:f64 = (newpos.position[0] - oldpos.position[0] * diffusion_coeff * dt * oldpos.qforce[0]).powi(2);
        let ydim:f64 = (newpos.position[1] - oldpos.position[1] * diffusion_coeff * dt * oldpos.qforce[1]).powi(2);
        let analytical: f64 = (-(xdim+ydim)/(4. * diffusion_coeff * dt)).exp();

        //Assertation
        let tol: f64 = 1E-13;
        assert!((BruteForceMetropolis::greens(&next_step[i], &system.particles[i]) - analytical) < tol);
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
        assert!((
            system.ham.energy(&system.wf, &mut system.particles, 1.0).unwrap()
            - system.ham.energy(&system.wf, &mut system.particles, 1.0).unwrap()
        ).abs() < tol);
        // Assertion
        //assert_eq!(smpldvls.map["energy"], d_energy);
        //assert_eq!(smpldvls.map["energy_sqrd"], d_energy.powi(2));
        assert!((smpldvls.map["wf_deriv"] - d_wf_deriv).abs() < tol);
        //assert_eq!(smpldvls.map["wf_deriv_times_energy"], d_wf_deriv * d_energy);
    }
}
