use rand::thread_rng;
use rand::distributions::{Uniform, Distribution};
use crate::{
    System,
    Particle,
    montecarlo::SampledValues,
};


pub enum MetropolisResult {
    Accepted(SampledValues),
    Rejected,
}

/// Trait for Metropolis samplers.
pub trait Metropolis {
    fn new(step_size: f64) -> Self;
    fn step(&mut self, sys: &mut System) -> MetropolisResult;
    fn hastings_check(acceptance_factor: f64) -> bool {
        if acceptance_factor >= 1. { true }
        else {
            let mut rng = thread_rng();
            let uniform = Uniform::new(0., 1.);
            uniform.sample(&mut rng) < acceptance_factor
        }
    }
    fn sample(sys: &mut System) -> SampledValues {
        let d_wf_deriv = sys.wavefunction.gradient_alpha(&sys.particles); 
        // The 1.0 inputted below is a placeholder for the omega value. We are testing over
        // different omega values. TODO: Consider storing omega in the System struct instead of
        // passing it through the stack.
        let d_energy = sys.hamiltonian.energy(&sys.wavefunction, &mut sys.particles, 1.0);

        SampledValues {
            energy: d_energy,
            energy_squared: d_energy.powi(2),
            wf_deriv: d_wf_deriv,
            wf_deriv_times_energy: d_wf_deriv * d_energy,
            accepted_steps: 0,
        }
    }
    fn greens(x: &Particle, y: &Particle) -> f64 {
        let mut result: f64 = 0.;
        for j in 0..x.dim { // This is a vector sum + scalar product
            // 0.0025 here is the same as 0.5 * 0.005
            result += (x.position[j] - y.position[j] - 0.0025 * y.qforce[j]).powi(2);
        }
        result = (- result / 0.01).exp(); // Ignoring denominator of Greens since it cancels later
        result
    }
}


pub struct BruteForceMetropolis {
    step_size: f64,
}

impl Metropolis for BruteForceMetropolis {
    fn new(step_size: f64) -> Self {
        Self { step_size, }
    }

    fn step(&mut self, sys: &mut System) -> MetropolisResult {
        // Make a step
        let next_step = sys.random_particle_change(self.step_size);

        // Evaluate wavefunction for old and new states
        let wf_old: f64 = sys.wavefunction.evaluate(&sys.particles);
        let wf_new: f64 = sys.wavefunction.evaluate(&next_step);

        if Self::hastings_check(wf_new.powi(2) / wf_old.powi(2)) {
            sys.particles = next_step;
            MetropolisResult::Accepted(Self::sample(sys))
        } else {
            MetropolisResult::Rejected
        }
    }
}


pub struct ImportanceMetropolis;

impl Metropolis for ImportanceMetropolis {
    fn new(_: f64)  -> Self { Self }

    fn step(&mut self, sys: &mut System) -> MetropolisResult {
        // Make a step
        let (next_step, i) = sys.quantum_force_particle_change();

        // Evaluate wavefunction for old and new states
        let wf_old: f64 = sys.wavefunction.evaluate(&sys.particles);
        let wf_new: f64 = sys.wavefunction.evaluate(&next_step);

        // Calculate the acceptance factor
        let greens_factor = Self::greens(&sys.particles[i], &next_step[i]) / Self::greens(&next_step[i], &sys.particles[i]);
        let acceptance_factor = greens_factor * wf_new.powi(2) / wf_old.powi(2);

        if Self::hastings_check(acceptance_factor) {
            sys.particles = next_step;
            MetropolisResult::Accepted(Self::sample(sys))
        } else {
            MetropolisResult::Rejected
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hastings_check() {
        assert!(BruteForceMetropolis::hastings_check(1. as f64));    //Panics if it returns false
        assert!(BruteForceMetropolis::hastings_check(2.));
        assert!(!BruteForceMetropolis::hastings_check(0.))    //Panics if it returns true
    }
}
