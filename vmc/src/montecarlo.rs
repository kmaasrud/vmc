use crate::{Metropolis, System};
use std::collections::HashMap;
use std::ops::AddAssign;

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

    pub fn divide_f64(&mut self, factor: f64) {
        for val in self.map.values_mut() {
            *val /= factor;
        }
    }
}

impl AddAssign for SampledValues {
    fn add_assign(&mut self, other: Self) {
        for (key, val) in self.map.iter_mut() {
            *val += other.map[key];
        };
    }
}

/// Does Monte Carlo integration over the WaveFunction of a System, using a given Metropolis
/// algorithm.
pub fn monte_carlo<T: Metropolis>(n: usize, sys: &mut System, metro: &mut T) -> Result<SampledValues, String> {
    let pre_steps = n / 4;
    let mut result = SampledValues::new();

    // Run a couple of steps to get the system into equilibrium
    for _ in 0..pre_steps {
        match metro.step(sys)? {
            Some(vals) => result = vals,
            None => {}
        }
    }

    // Store the previous values to add if Metropolis step is rejected
    let mut prev_dvals = result.clone();
    for _ in 0..n {
        match metro.step(sys)? {
            Some(dvals) => {
                result += dvals;
                prev_dvals = dvals;
            }
            None => {
                result += prev_dvals;
            }
        }
    }

    // Divide all values by n to get the mean
    result.divide_f64(n as f64);
    Ok(result)
}
