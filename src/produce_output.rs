use crate::{
    monte_carlo,
    ThreadPool,
    System,
    BruteForceMetropolis,
    ImportanceMetropolis,
    WaveFunction,
    Hamiltonian,
    Metropolis,
};

use std::{
    time::Instant,
    fs::{File, create_dir_all},
    path::Path,
    io::prelude::*,
    
};
use num_cpus;



/// Produces results for dimensions 1-3, different alphas and different number of particles and
/// saves these in its own separate file. Does this a number of times corresponding to the number
/// of cores the CPU running the program has.
pub fn _dim_and_n() {
    const CSV_HEADER: &str = "Alpha,Energy,Energy2,TimeElapsed\n";
    const STEP_SIZE: f64 = 1.0;
    //const ALPHAS: [f64; 8] = [0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    const MC_CYCLES: usize = 1_000;

    fn analytical(sys: &System)  -> f64{
        let dim = sys.dimensionality;
        let n = sys.particles.len();
        let alpha = sys.wavefunction.alpha;
        let particles = &sys.particles;

        let squared_position_sum: f64 = particles.iter().map(|x| x.squared_sum()).sum();
        let energy =  (alpha as f64) * (n as f64) * (dim as f64) + (0.5  - 2. * (alpha as f64).powf(2.)) * (squared_position_sum as f64);
        return energy
    }

    fn run_sim(start: Instant, mc_cycles: usize) {
        let path = format!("./data/numerical/dim_and_n/{:?}/", std::thread::current().id());
        let path_ana = format!("./data/analytical/dim_and_n/{:?}/", std::thread::current().id());
        create_dir(&path);
        create_dir(&path_ana);
         
        let mut metro: BruteForceMetropolis = BruteForceMetropolis::new(STEP_SIZE);

        for dim in 1..=3 {
            for n in [1].iter() {
                println!("Thread {:?} is calculating -- Dimensionality: {} --  Number of particles: {}", std::thread::current().id(), dim, n);

                let mut f = create_file(&format!("{}/numerical_{}D_{}_n_part.csv", &path, dim, n));
                let mut a = create_file(&format!("{}/analytical_{}D_{}_n_part.csv", &path_ana, dim, n));
                f.write_all(CSV_HEADER.as_bytes()).expect("Unable to write data"); 
                a.write_all(CSV_HEADER.as_bytes()).expect("Unable to write data"); 

                //for alpha in ALPHAS.iter() {
                for alpha in 0..= 90 {
                    let alpha = alpha as f64 * 0.01;
                    let ham: Hamiltonian = Hamiltonian::spherical();
                    let wf = WaveFunction{ alpha: alpha, beta: 1. }; // Beta = 1, because spherical trap
                    let mut system: System = System::distributed(*n, dim, wf, ham, 1.);
                    let vals = monte_carlo(mc_cycles, &mut system, &mut metro); 
                    
                    let energy_exact = analytical(&system);
                    let energy_exact_squared = energy_exact.powi(2);
                    
                    
                    let duration = start.elapsed();
                    let data_n = format!("{},{},{},{:?}\n", alpha, vals.energy, vals.energy_squared, duration);
                    let data_a = format!("{},{},{},{:?}\n", alpha, energy_exact, energy_exact_squared, duration);
                    
                    f.write_all(data_n.as_bytes()).expect("Unable to write data");
                    a.write_all(data_a.as_bytes()).expect("Unable to write data");
                    println!("Dimension: {} --- Alpha: {:.1} --- N: {:.2} --- Energy: {} --- Analytical: {:.2}", dim, alpha, n, vals.energy, energy_exact);
                }
            }
        }
    }

    let n_cpus = num_cpus::get();
    println!("Found {} cores!", n_cpus);

    let mc: usize = MC_CYCLES / n_cpus;
    println!("Running {} Monte Carlo cycles on each core.", mc);

    let pool = ThreadPool::new(n_cpus as u8);

    let start = Instant::now();
    for _ in 0..n_cpus {
        pool.execute(move || run_sim(start, mc));
    }

    println!("All cores now executing, waiting for them to finish...");
    pool.join_all();

    println!("Total time spent: {:?}", start.elapsed());
}





/// Runs the VMC for dimension 1-3, different values of alpha and different step sizes. 
/// Does this using both brute force Metropolis sampling and importance Metropolis sampling.
pub fn _bruteforce_vs_importance() {
    const N: usize = 50;
    const ALPHAS: [f64; 8] = [0.3, 0.35, 0.4, 0.45, 0.5, 0.55, 0.6, 0.65];
    const MC_CYCLES: usize = 1000;
    const CSV_HEADER: &str = "StepSize,Alpha,Energy,Energy2\n";

    fn run_sim<T: Metropolis>(mc: usize, step_size: f64) {
        for dim in 1..=3 {
            let path = format!("./data/bruteforce_vs_importance/{}/step_size{}", std::any::type_name::<T>().split("::").last().unwrap(), step_size);
            create_dir(&path);

            let mut f = create_file(&format!("{}/{}D.csv", &path, dim));
            f.write_all(CSV_HEADER.as_bytes()).expect("Unable to write data");

            for alpha in ALPHAS.iter() {
                let ham: Hamiltonian = Hamiltonian::elliptical(2.82843); // Input value is gamma
                let wf = WaveFunction{ alpha: *alpha, beta: 2.82843 }; // Set beta = gamma
                let mut system: System = System::distributed(N, dim, wf, ham, 1.);
                let mut metro: T = T::new(step_size);
                let vals = monte_carlo(mc, &mut system, &mut metro); 

                let data = format!("{},{},{},{}\n", step_size, alpha, vals.energy, vals.energy_squared);
                f.write_all(data.as_bytes()).expect("Unable to write data");
                println!("Dimension: {} --- Alpha: {:.1} --- Step size: {:.2} --- Energy: {}", dim, alpha, step_size, vals.energy);
            }
        }
    }

    fn run_for_sampler<T: Metropolis>() {
        println!("Running simulations using {} algorithm...", std::any::type_name::<T>().split("::").last().unwrap());

        // Multithreading
        let n_cpus = num_cpus::get();

        println!("Spawning threadpool of {} threads, with {} Monte Carlo cycles on each", &n_cpus, &MC_CYCLES);
        let pool = ThreadPool::new(n_cpus as u8);
        let start = Instant::now();

        for cpu_i in 1..=n_cpus {
            pool.execute(move || run_sim::<T>(MC_CYCLES, (cpu_i as f64) / (n_cpus as f64))); //Running the simulation on each thread individually
        }
        println!("All {} threads now executing, waiting for them to finish...", n_cpus);
        pool.join_all();
        
        println!("Time spent: {:?}", start.elapsed());
    }

    // run_for_sampler::<BruteForceMetropolis>();
    run_sim::<ImportanceMetropolis>(MC_CYCLES, 1.); // Step size not relevant here, so 1. does nothing
}

/// Runs the VMC for dimension X, utilizing simple gradient descent in order to choose fitting alpha parameter.
/// Only done using the noninteracting case, with importance sampling
pub fn sgd_noninteracting() {
    //DINGDINGDING, DO THE WORK!
    const N: usize = 10;
    const MC_CYCLES: usize = 10000;
    const CSV_HEADER: &str = "StepSize,Alpha,Energy,Energy2\n";

    let mut alphas:Vec<f64> = vec![];
    let mut energies:Vec<f64> = vec![];

    alphas.push(0.1); alphas.push(0.2);

    let mut done: bool = false;
    let tolerance: f64 = 0.000_000_001;
    
    let dim: usize = 3;
    let step_size: f64 = 1.;



    println!("Running simulations using ImportanceMetropolis algorithm...");
    let start = Instant::now();

    let mut i:usize = 0;
    while !done {

        let path = format!("./data/sdg_noninteracting/step_size{}", step_size);
        create_dir(&path);
        let mut f = create_file(&format!("{}/{}-alpha.csv", &path, &alphas[i]));
        f.write_all(CSV_HEADER.as_bytes()).expect("Unable to write data");

        let ham: Hamiltonian = Hamiltonian::elliptical(2.82843); // Input value is gamma
        let wf = WaveFunction{ alpha: alphas[i], beta: 2.82843 }; // Set beta = gamma
        let mut system: System = System::distributed(N, dim, wf, ham, 1.);
        let mut metro: BruteForceMetropolis = BruteForceMetropolis::new(step_size);
        let vals = monte_carlo(MC_CYCLES, &mut system, &mut metro); 

        energies.push(vals.energy);

        let data = format!("{},{},{},{}\n", step_size, alphas[i], vals.energy, vals.energy_squared);
        f.write_all(data.as_bytes()).expect("Unable to write data");
        println!("Dimension: {} --- Alpha: {} --- Step size: {:.2} --- Energy: {}", dim, alphas[i], step_size, vals.energy);
   
        if i > 0 {
            let variance: f64 = vals.energy_squared-vals.energy;
            let new_alpha: f64 = alphas[i] - 1000.* (2.* (vals.wf_deriv_times_energy-vals.wf_deriv*vals.energy));
            println!("             New Alpha: {}", &new_alpha);
            alphas.push(new_alpha);

            if variance < tolerance {
                done = true;
            }
            //if (energies[i]-energies[i-1]).abs() < tolerance {
            //    done = true;
            //}
        }

        

        i += 1;
    }

        
    println!("Time spent: {:?}", start.elapsed());

}

fn create_file(filepath: &str) -> File {
    match File::create(&Path::new(filepath)) {
        Ok(f) => f,
        Err(why) => panic!("Unable to create {}: {}", filepath, why),
    }
}

fn create_dir(path: &str) {
    if Path::new(&path).exists() == false {
        create_dir_all(&path).expect("Unable to create folder");
    }
}
