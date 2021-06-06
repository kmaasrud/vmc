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
        let (energy, kinetic) = Hamiltonian::energy(&sys)?;
        let wf_deriv_alpha = sys.wf.gradient_alpha(&sys.particles)?;
        let wf_deriv_beta = sys.wf.gradient_beta(&sys.particles)?;

        let mut map = HashMap::new();
        map.insert("energy".to_string(), energy);
        map.insert("kinetic".to_string(), kinetic);
        map.insert("energy_sqrd".to_string(), energy.powi(2));
        map.insert("wf_deriv_alpha".to_string(), wf_deriv_alpha);
        map.insert("wf_deriv_alpha_times_energy".to_string(), wf_deriv_alpha * energy);
        map.insert("wf_deriv_beta".to_string(), wf_deriv_beta);
        map.insert("wf_deriv_beta_times_energy".to_string(), wf_deriv_beta * energy);
        Ok(SampledValues { map, accepted_steps: 0 })
    }

    fn greens(x: &Particle, y: &Particle, n: usize) -> Result<f64, String> {
        let diffusion = 0.5;
        let time_step = 0.001;
        let factor = 1. / (4. * diffusion * time_step);
        let yx = y.position - x.position - x.qforce.scale(diffusion * time_step);
        let xy = x.position - y.position - y.qforce.scale(diffusion * time_step);
        let vec_sum = x.position - y.position - y.qforce.scale(0.0025);
        // Ignoring denominator of Greens since it cancels later
        // Ok(((- xy.inner(xy)? + yx.inner(yx)?) * factor + n as f64 - 1.).exp())
        Ok((-vec_sum.inner(vec_sum)? * factor).exp())
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
                let wf_old = sys.wf.evaluate::<N>(&sys.particles)?;
                let wf_new = sys.wf.evaluate::<N>(&new_particles)?;
                wf_new.powi(2) / wf_old.powi(2)
            }
            _ => {
                new_inverse = sys.next_slater_inverse(&new_particles, p)?;
                if sys.wf.jastrow_on {
                    sys.next_slater_ratio(p, &new_inverse)
                } else {
                    sys.next_slater_ratio(p, &new_inverse) * sys.next_jastrow_ratio(p, &new_particles)
                }
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
        let n = sys.particles.len();

        // Calculate the acceptance factor
        let greens_factor = Self::greens(&sys.particles[p], &new_particles[p], n)?
            / Self::greens(&new_particles[p], &sys.particles[p], n)?;

        let acceptance_factor = match N {
            2 => {
                let wf_old = sys.wf.evaluate::<N>(&sys.particles)?;
                let wf_new = sys.wf.evaluate::<N>(&new_particles)?;
                //println!("GF: {:.16} || wfN: {:.16} || wfO: {:.16}", greens_factor, wf_new, wf_old);
                greens_factor * wf_new.powi(2) / wf_old.powi(2)
            }
            _ => {
                new_inverse = sys.next_slater_inverse(&new_particles, p)?;
                greens_factor * if !sys.wf.jastrow_on {
                    sys.next_slater_ratio(p, &new_inverse)
                } else {
                    sys.next_slater_ratio(p, &new_inverse) * sys.next_jastrow_ratio(p, &new_particles)
                }
            }
        };
        //println!("{}", acceptance_factor);

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
    use crate::Vector;

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
        let mut pnew = Particle::from_vector(Vector::D2(0.011, 0.011));
        pnew.qforce += Vector::D2(0.2, 0.2);

        // a = 1, alpha = 0.5 omega = 1, beta = 1
        let r12: f64 = p0.distance_to(&pold).unwrap();
        let qforce_vec = p0.position.scale(-2. * 0.5)
            + (p0.position - pold.position).scale(2. / (r12 * (1. + r12).powf(2.)))
            + pold.position.scale(-2. * 0.5)
            + (pold.position - p0.position).scale(2. / (r12 * (1. + r12).powf(2.)));
        let relpos = pnew.position - pold.position;
        let poldscaled = pold.position.scale(diffusion_coeff * dt);
        let therest = (relpos - poldscaled)
            .scale(-1.)
            .scale(1. / (4. * diffusion_coeff * dt)); //Fuck me for doing this ugly shit
        let analytical: f64 = therest.inner(therest).unwrap();

        //Assertation
        let tol: f64 = 1E-12;
        assert_eq!(BruteForceMetropolis::greens(&pnew, &pold).unwrap(), analytical);
        // assert!((BruteForceMetropolis::greens(&pnew, &pold).unwrap() - analytical) < tol);
    }
}
