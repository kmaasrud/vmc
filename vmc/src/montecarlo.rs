use crate::{Metropolis, System, utils};
use std::collections::HashMap;
use std::io::Write;
use std::time::Instant;

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

    //Save to file
    let mut path = utils::find_cargo_root().unwrap();
    path.push("data");
    utils::create_dir(&path);
    let metro_type = std::any::type_name::<T>().split("::").last().unwrap();
    path.push(format!("E_vs_MCs_{}.csv", metro_type));
    let mut f = utils::create_file(&path);
    f.write_all("MCcycle,energy[au],time[s]\n".as_bytes()).expect("Unable to write data");
    let start = Instant::now();


    // Run a couple of steps to get the system into equilibrium
    for i in 0..pre_steps {
        match metro.step(sys)? {
            Some(vals) => result = vals,
            None => {}
        }

    }

    // Store the previous values to add if Metropolis step is rejected
    let mut prev_dvals = result.clone();
    for i in 0..n {
        match metro.step(sys)? {
            Some(dvals) => {
                result.add_to_sum(&dvals);
                prev_dvals = dvals;
                
                //Writing to file
                let data = format!("{},{},{}\n",i, result.map.get("energy").unwrap() / (i as f64), start.elapsed().as_millis() as f64 / 1000.);
                f.write_all(data.as_bytes()).expect("Unable to write data");
            }
            None => {
                result.add_to_sum(&prev_dvals);
            }
        }
    }

    // Divide all values by n to get the mean
    result.divide_f64(n as f64);
    Ok(result)
}
