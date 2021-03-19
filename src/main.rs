mod particle;
mod metropolis;
mod system;
mod wavefunction;
mod hamiltonian;
mod montecarlo;
mod threadpool;
mod analytical;

pub use particle::Particle;
pub use system::System;
pub use metropolis::{Metropolis, MetropolisResult, BruteForceMetropolis};
pub use wavefunction::{WaveFunction, GaussianWaveFunction};
pub use hamiltonian::{Hamiltonian, HarmonicOscillator};
use montecarlo::monte_carlo;
use threadpool::ThreadPool;
use analytical::local_energy_analytical;

use std::time:: Instant;

extern crate num_cpus;

use std::sync::atomic::{AtomicUsize, Ordering};

use std::fs::OpenOptions;
use std::io::{BufWriter, Write};


fn main() {
    //let alpha = 0.5;
    //let n_particles = 1 ;
    //let dimensions = 1;
    let step_size = 1.0;
    let mc_cycles = 1_000;
    


    let alpha_list: Vec<f64> = vec![0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    let max_dim: usize = 3;
    let particle_list = vec![1,10,100];

    let variance = 1;
    let accept_ratio = 1;

    let header = "Alpha,Energy,Energy2,Variance,AcceptRatio,ElapesdTime\n";
    
    let start = Instant::now();
    let cpus = 1; //num_cpus::get();
    let pool = ThreadPool::new(cpus as u8);
    for _ in 0..cpus {
        let alc = alpha_list.clone();
        let plc = particle_list.clone();
        pool.execute(move ||run_sim(alc.clone(), plc.clone(), variance, accept_ratio, header, start, step_size, mc_cycles));
    }
    //run_sim(alpha_list, particle_list, variance, accept_ratio, header, start, step_size, mc_cycles);
    println!("All cores now executing, calling join and waiting for end");
    pool.join_all();

    println!("Total time spent: {:?}", start.elapsed());


}



fn run_sim(alpha_list: Vec<f64>, particle_list:Vec<usize>, variance:i32, accept_ratio:i32, header: &str, start:Instant, step_size:f64, mc_cycles:usize) {
    for dim in 1..=3{
        //println!("dim: {}",dim);

        for particle in particle_list.iter(){
            println!("Calculating: dim: {}, n_part {}, {:?}",dim, particle, std::thread::current().id());
            //dummypath
            //let path = format!("./data/dummydata/dummy_{}D_{}_particles.csv", dim, particle);
            //path for analytical results
            //let path = format!("./data/analytic/experiment_{}D_{}_particles_ana.csv", dim, particle);
            //path numerical results
            
            let f = OpenOptions::new()
            let path = format!("./data/non_paralell/numeric/experiment_{}D_{}_particles_num_{:?}.csv", dim, n, std::thread::current().id());
            let path_ana = format!("./data/non_paralell/analytic/experiment_{}D_{}_particles_num_{:?}.csv", dim, n, std::thread::current().id());
            let f_ile = OpenOptions::new()
                        .read(true)
                        .append(true)
                        .create(true)
                        .open(path)
                        .expect("Unable to open file");
            let mut f = BufWriter::new(f_ile);
            let mut f_ana = BufWriter::new(f_ile);
            
            f.write_all(header.as_bytes()).expect("Unable to write data"); 
            f_ana.write_all(header.as_bytes()).expect("Unable to write data"); 


            for alpha in alpha_list.iter(){
                let wf: GaussianWaveFunction = GaussianWaveFunction::new(*alpha);
                let ham: HarmonicOscillator = HarmonicOscillator::elliptical(1.0, 1.0);
                let mut test_system: System<GaussianWaveFunction, HarmonicOscillator> = System::distributed(*n, dim, wf, ham, 0.1);
                let mut metro: BruteForceMetropolis = BruteForceMetropolis::new(step_size);
                //println!("Energy from monte carlo calculations {}", monte_carlo(mc_cycles, &mut test_system, &mut metro)); 
                let energy = monte_carlo(mc_cycles, &mut test_system, &mut metro); 
                let energy2 = energy.powi(2);

                let energy_ana = local_energy_analytical(alpha, dim, &test_system.particles);
                let energy2_ana = energy_ana.powi(2);

                let duration = start.elapsed();
                //println!("Time used in seconds {:?} = {:?} min",duration, duration/60);
                let duration = start.elapsed();
                let data = format!("{},{},{},{},{},{:?}\n", alpha, energy, energy2, variance, accept_ratio, duration);
                let data_ana = format!("{},{},{},{},{},{:?}\n", alpha, energy_ana, energy2_ana, variance, accept_ratio, duration);
                
                f.write_all(data.as_bytes()).expect("Unable to write data");
                f_ana.write_all(data_ana.as_bytes()).expect("Unable to write data");
            }
        }
    }
}
