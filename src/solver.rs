use rand::thread_rng;
use rand::distributions::{Uniform, Distribution};

/// Trait for Metropolis samplers. 
trait Metropolis {
    /// Evaluates whether or not a step should be takeng, based on the
    /// current curr_step and next_step. Returns a Boolean.
    fn do_change(&mut self) -> bool {
        let mut rng = thread_rng();
        let uniform = Uniform::new(0., 1.);
        
        if uniform.sample(&mut rng) < self.acceptance_factor() {
            true
        } else {
            false
        }
    }
    
    fn step(&mut self) -> &Vec<f64>;
    fn acceptance_factor(&self) -> f64;
    fn next_step(&mut self);
    fn greens_function(&self) -> f64;
}

/// Struct for representing a brute force Metropolis algorithm.
/// Implements the Metropolis trait.
pub struct BruteForceMetropolis {
    // `curr_step` describes the N particles with D dimensions as a N * D dimensional vector.
    // This is computationally preferrable and allows for more general code. Do keep this in mind.
    curr_step: Vec<f64>,
    next_step: Vec<f64>,
    step_size: f64,
}

impl BruteForceMetropolis {
    /// Makes a new `BruteForceMetropolis` struct based on a step size.
    fn new(step_size: f64) -> Self {
        // Initialize with random `curr_step`. Just setting empty for now, so the vector needs to be filled.
        Self{ curr_step: vec![], next_step: vec![], step_size: step_size, }
    }

}

impl Metropolis for BruteForceMetropolis {
    /// Makes a new step based on `curr_step`. Also updates
    /// the value of `self.curr_step`.
    fn step(&mut self) -> &Vec<f64> {
        self.next_step();
        if self.do_change() {
            &self.curr_step
        } else {
            &self.next_step
        }
    }
    /// Calculates the acceptance factor based on the current step (stored in the struct) and the next step. 
    fn acceptance_factor(&self) -> f64 {
        // TODO: We need WaveFunction structs, the below is just random rubbish now
        let wave_function_old: f64 = self.curr_step.iter().sum();
        let wave_function_new: f64 = self.next_step.iter().sum();
        
        // Not random rubbish anymore
        let hastings_ratio: f64 = wave_function_new.powi(2) / wave_function_old.powi(2);

        // Return hastings ratio if it is smaller than 1, else 1
        hastings_ratio.min(1.)
    }
    
    /// This is what makes this a brute force method, as `BruteForceMetropolis` only makes
    /// a random step in either direction. 
    fn next_step(&mut self) {
        // thread_rng() randomly chooses either `1.` or `-1.`. Don't know if this is the most efficient way, but it should work...
        let uniform = Uniform::new(0., 1.);
        let mut next_step: Vec<f64> = vec![];
        for i in 0..self.curr_step.len() {
            // This is what Morten does in [this example](https://compphysics.github.io/ComputationalPhysics2/doc/pub/week2/html/week2.html#___sec11)
            // I trust him being correct, but I am not totally sure about the `- 0.5` part...
            next_step.push(self.curr_step[i]
                + (uniform.sample(&mut thread_rng()) - 0.5) * self.step_size);
        }
        self.next_step = next_step;
    }
}

pub struct ImportanceMetropolis {
    curr_step: Vec<f64>,
    next_step: Vec<f64>,
    step_size: f64,
}

impl ImportanceMetropolis {
    /// Makes a new `ImportanceMetropolis` struct based on a step size.
    fn new(step_size: f64) -> Self {
        // Initialize with random `curr_step`. Just setting empty for now, so the vector needs to be filled.
        Self{ curr_step: vec![], next_step: vec![], step_size: step_size, }
    }
}

impl Metropolis for ImportanceMetropolis {
    /// Makes a new step based on `curr_step`. Also updates
    /// the value of `self.curr_step`.
    fn step(&mut self) -> &Vec<f64> {
        self.next_step();
        if self.do_change() {
            &self.curr_step
        } else {
            &self.next_step
        }
    }
    /// Calculates the acceptance factor based on the current step (stored in the struct) and the next step. 
    fn acceptance_factor(&self) -> f64 {
        // TODO: We need WaveFunction structs, the below is just random rubbish now
        let wave_function_old: f64 = self.curr_step.iter().sum();
        let wave_function_new: f64 = self.next_step.iter().sum();
        
        // Not random rubbish anymore
        let hastings_ratio: f64 = wave_function_new.powi(2) / wave_function_old.powi(2);

        // Return hastings ratio if it is smaller than 1, else 1
        hastings_ratio.min(1.)
    }
    
    fn greens_function(&self) -> f64 {
        let atomic_force: f64 = 2.*1./(self.curr_step.iter().sum())*self.curr_step.iter().sum();
        let greens: f64;
        let N:i64 = 2;
        for i in 0..self.curr_step.len(){
            greens += 1./((4.*3.14*0.5*self.step_size).powf(3.*N as f64 / 2.)) * ((-(self.next_step[i]-self.curr_step[i]-0.5*self.step_size*atomic_force).powi(2))/(4.*0.5*self.step_size));
            }
        greens
    }


    /// This is what makes this a brute force method, as `BruteForceMetropolis` only makes
    /// a random step in either direction. 
    fn next_step(&mut self) {
        // thread_rng() randomly chooses either `1.` or `-1.`. Don't know if this is the most efficient way, but it should work...
        let uniform = Uniform::new(0., 1.);
        let mut next_step: Vec<f64> = vec![];
        for i in 0..self.curr_step.len() {
            // This is what Morten does in [this example](https://compphysics.github.io/ComputationalPhysics2/doc/pub/week2/html/week2.html#___sec11)
            // I trust him being correct, but I am not totally sure about the `- 0.5` part...
            next_step.push(self.curr_step[i]
                + (uniform.sample(&mut thread_rng()) - 0.5) * self.step_size);
        }
        self.next_step = next_step;
    }
}