use crate::{Metropolis, System};
use std::collections::HashMap;

/// Collection of values that are integrated over
#[derive(Clone, Debug)]
pub struct SampledValues {
    pub map: HashMap<String, f64>,
}

impl SampledValues {
    pub fn new() -> Self {
        SampledValues {
            map: HashMap::new(),
        }
    }

    pub fn add_to_sum(&mut self, dvals: &SampledValues) {
        for (key, val) in self.map.iter_mut() {
            *val += dvals.map[key];
        }
    }

    pub fn divide_f64(&mut self, factor: f64) {
        for val in self.map.values_mut() {
            *val /= factor;
        }
    }
}

/// Does Monte Carlo integration over the WaveFunction of a System, using a given Metropolis
/// algorithm.
pub fn monte_carlo<T: Metropolis, const N: usize>(
    n: usize,
    sys: &mut System<N>,
    metro: &mut T,
) -> Result<SampledValues, String> {
    let pre_steps = n / 4;
    let mut result = SampledValues::new();

    // Run a couple of steps to get the system into equilibrium
    for i in 0..pre_steps {
        match metro.step(sys)? {
            Some(vals) => result = vals,
            None => {}
        }

        println!("{}", i);
    }

    // Store the previous values to add if Metropolis step is rejected
    let mut prev_dvals = result.clone();
    for i in 0..n {
        match metro.step(sys)? {
            Some(dvals) => {
                result.add_to_sum(&dvals);
                prev_dvals = dvals;
            }
            None => {
                result.add_to_sum(&prev_dvals);
            }
        }
        println!("{}", i);
    }

    // Divide all values by n to get the mean
    result.divide_f64(n as f64);
    Ok(result)
}
