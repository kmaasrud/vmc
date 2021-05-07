use crate::Particle;
//use crate::{Hamiltonian, Particle};


#[derive(Clone)]
pub struct WaveFunction {
    pub alpha: f64,
    pub beta: f64,
    pub a: f64,
}

impl WaveFunction {
    //-- Trial wavefunction --
    /// Trial wavefunction for the ground state of the two electron/fermion system. 
    /// Returns an f64 representing the wavefunction value
    pub fn evaluate(&self, particles: &Vec<Particle>) -> f64 {
        let omega: f64  = 1.0;
        let c: f64      = 1.0 ; //normalization constant - dont know value

        match particles.len() {
            // In the case of two particles, evaluating the wavefunction is a bit simpler.
            2 => {
                let mut exp_sum = 0.;
                for (i, particle) in particles.iter().enumerate(){
                    for other in particles[i+1..].iter(){
                        let fermion_distance :f64 = particle.distance_to(other);
                        exp_sum += self.a * fermion_distance / (1. + self.beta * fermion_distance);
                    } 
                }
                
                let r1: f64 = particles[0].squared_sum();
                let r2: f64 = particles[1].squared_sum();

                let result: f64 = c * (-0.5  * self.alpha * omega * (r1 + r2) + exp_sum).exp();
                
                result
            },
            // This is the general evaluation, using Slater determinants
            _ => {
                1.
            }
        }
    }

     // --- Laplacian ---
    /// Returns the Laplacian of the wavefunction evaluated numerically at state of 'particles'.
    pub fn laplace(&self, particles: &mut Vec<Particle>) -> f64 {
        let h: f64 = 0.0001; //stepsize
        let h2 = h.powi(2);

        let mut laplace = 0.;

        let wf = self.evaluate(&particles);

        for i in 0..particles.len() {
            for dim in 0..particles[i].dim {
                particles[i].bump_at_dim(dim, h); // Initial position +h
                let wf_plus = self.evaluate(particles);

                particles[i].bump_at_dim(dim, -2. * h); // Initial position -h
                let wf_minus = self.evaluate(particles);

                particles[i].bump_at_dim(dim, h); // Reset back to initial position

                laplace += (wf_plus - 2. * wf + wf_minus) / h2; 
            }
        }
        laplace / wf
    }
   

    // --- Gradients ---
    /// Returns the gradient for a particle with regards to the non-interacting part of the
    /// wavefunction
    fn gradient_spf(&self, particle: &Particle) -> Vec<f64> {
        let mut gradient = particle.position.clone();
        if gradient.len() > 2 { gradient[2] *= self.beta; }
        gradient.iter().map(|x| - 2. * self.alpha * x).collect()
    }
    /// Returns the gradient for a particle with regards to the interaction-part of the
    /// wavefunction
    fn gradient_interaction(&self, i: usize, particles: &Vec<Particle>) -> Vec<f64> {
        let mut gradient = vec![0.; particles[i].dim];
        let a: f64 = 0.0043;

        for j in 0..particles.len() {
            if i == j { continue }
            let distance: f64 = particles[i].distance_to(&particles[j]);
            for dim in 0..particles[i].dim {
                gradient[dim] += a * (particles[i].position[dim] - particles[j].position[dim]) / (distance.powi(2) * (distance - a));
            }
        }
        gradient
    }
    /// Returns the gradient of the wavefunction with regards to alpha
    pub fn gradient_alpha(&self, particles: &Vec<Particle>) -> f64 {
        let squared_position_sum_sum: f64 = particles.iter().map(|x| x.squared_sum_scaled_z(self.beta)).sum();
        - squared_position_sum_sum
    }

    // --- Quantum forces ---
    pub fn quantum_force(&self, i: usize, particles: &Vec<Particle>) -> Vec<f64> {
        let quantum_force = self.gradient_spf(&particles[i]).iter()
            .zip(self.gradient_interaction(i, particles).iter())
            .map(|(x, y)| 2. * (x + y))
            .collect();
        quantum_force
    }
    /// Calculates the quantum force of a particle not interacting with its surrounding particles
    pub fn quantum_force_non_interacting(&self, particle: &Particle) -> Vec<f64> {
        self.gradient_spf(particle).iter().map(|x| 2. * x).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_laplace() {

    }

    #[test]
    fn test_evaluate_deterministicity() {
        use crate::{
            System,
            WaveFunction,
            Hamiltonian,
        };
        // Spawn a system with defined wavefunction and energy
        let ham: Hamiltonian = Hamiltonian;
        let wf = WaveFunction{ alpha: 0.5, beta: 1. , a: 1.}; // Set beta = gamma
        let system: System = System::distributed(10, 3, wf.clone(), ham.clone(), false, 1.);

        // Is it deterministic?
        assert_eq!(wf.evaluate(&system.particles), wf.evaluate(&system.particles));
    }

    #[test]
    fn test_evaluate_against_analytical() {
        use crate::{
            System,
            WaveFunction,
            Hamiltonian,
        };
        // System parameters
        let alpha:f64 = 0.5;
        let beta:f64 = 1.;
        let a:f64 = 1.;
        let omega:f64 = 1.;     //Defined separately in evaluate() function
        let c:f64 = 1.;         //Defined separately in evaluate() function

        // Spawn a system with defined wavefunction and energy
        let ham: Hamiltonian = Hamiltonian;
        let wf = WaveFunction{ alpha: alpha, beta: beta , a: a}; // Set beta = gamma
        let mut system: System = System::distributed(2, 2, wf.clone(), ham.clone(), false, 1.);
        system.particles[0].position = vec![0. ,0. ]; //Just placing the particles at specific positions
        system.particles[1].position = vec![1. ,1. ];
        println!("{:?}", system.particles);

        // Define the analytical answer to this problem

        let analytical = c * (-alpha * omega * (0. + 1.*1. + 1.*1.) / 2.).exp() 
                            * (a*((1.*1.+1.*1.) as f64).sqrt()/(1.+beta*((1.*1.+1.*1.) as f64).sqrt())).exp();
        println!("{}", analytical);
        // Assertation
        let tol:f64 = 1E-13;
        assert_eq!(wf.evaluate(&system.particles), analytical);
        assert!((wf.evaluate(&system.particles) - analytical).abs()<tol);
    }
}
