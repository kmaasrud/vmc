use crate::{
    montecarlo, BruteForceMetropolis, Hamiltonian, ImportanceMetropolis, Metropolis, System,
    ThreadPool, WaveFunction,
};

use std::{
    env,
    fs::{create_dir_all, File},
    io::prelude::*,
    path::{Path, PathBuf},
    time::Instant,
};

#[allow(dead_code)]
pub fn simple() {
    const ALPHA: f64 = 1.0;
    const OMEGA: f64 = 1.0;
    const BETA: f64 =  1.0;
    const JASTROW: bool = true;
    const STEP_SIZE: f64 = 0.1;
    const MC_CYCLES: usize = 100_000;
    const DIM: usize = 2;
    const N: usize = 2;
    const SPREAD: f64 = 0.1;

    fn simulate<T: Metropolis>(numerical_laplace: bool, interacting: bool) {
        let metro_type = std::any::type_name::<T>().split("::").last().unwrap();
        println!("Running run::simple() with {}, Numerical laplace: {:?}, Interacting: {:?}", &metro_type, &numerical_laplace, &interacting);
        let mut metro: T = T::new(STEP_SIZE);

        let mut path = find_cargo_root().unwrap();
        path.push("data");
        path.push("N2");
        create_dir(&path);

        let interact_str = if interacting { "interacting" } else { "non-interacting" };
        let numerical_str = if numerical_laplace { "numerical" } else { "analytical" };
        path.push(format!("{}_{}_{}-with-jastrow.csv", metro_type, interact_str, numerical_str));
        let mut f = create_file(&path);
        f.write_all("energy[au],time[s],kinetic,variance,acceptance_rate\n".as_bytes()).expect("Unable to write data");

        // Run 10 times
        for _ in 0..10 {
            let start = Instant::now();
            let wf = WaveFunction { alpha: ALPHA, beta: BETA, omega: OMEGA, jastrow_on: JASTROW }; // Set beta = gamma
            let mut system: System<N> = System::new(N, DIM, wf, interacting, numerical_laplace, SPREAD).unwrap();
            let vals = montecarlo::monte_carlo(MC_CYCLES, &mut system, &mut metro).unwrap();

            let energy = match vals.map.get("energy") {
                Some(val) => *val,
                None => 0.,
            };
            let energy_sqrd = match vals.map.get("energy_sqrd") {
                Some(val) => *val,
                None => 0.,
            };
            let kinetic = match vals.map.get("kinetic") {
                Some(val) => *val,
                None => 0.,
            };

            let acceptance_rate = (vals.accepted_steps as f64) / (MC_CYCLES as f64);
            let data = format!("{},{},{},{},{}\n", energy, start.elapsed().as_millis() as f64 / 1000., kinetic, energy_sqrd - energy.powi(2), acceptance_rate);
            f.write_all(data.as_bytes()).expect("Unable to write data");
            println!("{}", data);
        }
    }

    let start = Instant::now();
    let pool = ThreadPool::new(2);
    /*pool.execute(move || simulate::<BruteForceMetropolis>(false, false));
    pool.execute(move || simulate::<BruteForceMetropolis>(false, true));
    pool.execute(move || simulate::<BruteForceMetropolis>(true, false));
    pool.execute(move || simulate::<BruteForceMetropolis>(true, true));
    pool.execute(move || simulate::<ImportanceMetropolis>(false, false));
    pool.execute(move || simulate::<ImportanceMetropolis>(false, true));
    */
    pool.execute(move || simulate::<BruteForceMetropolis>(true, true));
    pool.execute(move || simulate::<ImportanceMetropolis>(true, true));
    pool.join_all();  
    println!("Total time spent: {:?}", start.elapsed());
}

#[allow(dead_code)]
pub fn multiple() {
    const JASTROW: bool = true;
    const NUMERICAL_LAPLACE: bool = true;
    const INTERACTING: bool = true;
    const STEP_SIZE: f64 = 0.1;
    const MC_CYCLES: usize = 100_000;
    const DIM: usize = 2;
    const SPREAD: f64 = 0.5;

    fn simulate<const N: usize>(omega: f64, alpha: f64, beta: f64) {
        let metro_type = "BruteForceMetropolis";
        println!("Running run::simple() with {}, Numerical laplace: {:?}, Interacting: {:?}", &metro_type, NUMERICAL_LAPLACE, INTERACTING);
        let mut metro = BruteForceMetropolis::new(STEP_SIZE);

        let mut path = find_cargo_root().unwrap();
        path.push("data");
        path.push(format!("N{}", N));
        create_dir(&path);

        path.push(format!("omega{}_alpha{}_beta{}.csv", omega, alpha, beta));
        let mut f = create_file(&path);
        f.write_all("energy[au],time[s],kinetic,variance,acceptance_rate\n".as_bytes()).expect("Unable to write data");

        // Run 5 times
        for _ in 0..5 {
            let start = Instant::now();
            let wf = WaveFunction { alpha, beta, omega, jastrow_on: JASTROW }; // Set beta = gamma
            let mut system: System<N> = System::new(N, DIM, wf, INTERACTING, NUMERICAL_LAPLACE, SPREAD).unwrap();
            let vals = montecarlo::monte_carlo(MC_CYCLES, &mut system, &mut metro).unwrap();

            let energy = match vals.map.get("energy") {
                Some(val) => *val,
                None => 0.,
            };
            let energy_sqrd = match vals.map.get("energy_sqrd") {
                Some(val) => *val,
                None => 0.,
            };
            let kinetic = match vals.map.get("kinetic") {
                Some(val) => *val,
                None => 0.,
            };

            let acceptance_rate = (vals.accepted_steps as f64) / (MC_CYCLES as f64);
            let data = format!("{},{},{},{},{}\n", energy, start.elapsed().as_millis() as f64 / 1000., kinetic, energy_sqrd - energy.powi(2), acceptance_rate);
            f.write_all(data.as_bytes()).expect("Unable to write data");
            println!("{}", data);
        }
    }

    let start = Instant::now();
    let pool = ThreadPool::new(10);
    pool.execute(move || simulate::<2>(0.01, 0.93, 0.16));
    pool.execute(move || simulate::<2>(0.05, 0.98, 0.24));
    pool.execute(move || simulate::<2>(0.1, 0.97, 0.35));
    pool.execute(move || simulate::<2>(0.5, 0.97, 0.38));
    pool.execute(move || simulate::<2>(1.0, 0.98, 0.43));
    /* pool.execute(move || simulate::<6>(0.01, 0.9, 0.05));
    pool.execute(move || simulate::<6>(0.05, 0.8, 0.15));
    pool.execute(move || simulate::<6>(0.1, 0.85, 0.2));
    pool.execute(move || simulate::<6>(0.5, 1.05, 0.25));
    pool.execute(move || simulate::<6>(1.0, 0.99, 0.5));
    pool.execute(move || simulate::<12>(0.01, 0.8, 0.5));
    pool.execute(move || simulate::<12>(0.05, 0.7, 0.15));
    pool.execute(move || simulate::<12>(0.1, 0.8, 0.2));
    pool.execute(move || simulate::<12>(0.5, 1.1, 0.5));
    pool.execute(move || simulate::<12>(1.0, 1.2, 0.4)); */
    pool.join_all();  
    println!("Total time spent: {:?}", start.elapsed());
}

#[allow(dead_code)]
pub fn sgd(interacting: bool) {
    const ALPHA: f64 = 1.0;
    const OMEGA: f64 = 1.0;
    const BETA: f64 = 0.;
    const JASTROW: bool = true;
    const STEP_SIZE: f64 = 0.1;
    const MC_CYCLES: usize = 100_000;
    const DIM: usize = 2;
    const N: usize = 2;
    const SPREAD: f64 = 0.1;
    const NUMERICAL_LAPLACE: bool = true;
    const TOLERANCE: f64 = 0.00001;

    fn simulate<T: Metropolis>(start_alpha:f64, start_beta:f64, learning_rate: f64, numerical_laplace: bool, interacting: bool) {
        let metro_type = std::any::type_name::<T>().split("::").last().unwrap();
        println!("Running run::sgd() with {}, Numerical laplace: {:?}, Interacting: {:?}, Start Alpha: {}, Start Beta: {}, Learning Rate: {}", &metro_type, &numerical_laplace, &interacting, &start_alpha, &start_beta, &learning_rate);
        let mut alphas:Vec<f64> = vec![];
        alphas.push(start_alpha);
        let mut betas:Vec<f64> = vec![];
        betas.push(start_beta);

        let mut metro: T = T::new(STEP_SIZE);

        let mut done: bool = false;
        let mut energies:Vec<f64> = vec![];

        let mut path = find_cargo_root().unwrap();
        path.push("data"); path.push("sgd"); path.push("start_params");
        create_dir(&path);
        path.push(format!("a-{}_b-{}.csv", start_alpha, start_beta));
        let mut f = create_file(&path);
        f.write_all("alpha,beta,energy-per-particle[au],time[s],variance\n".as_bytes()).expect("Unable to write data");

        let mut i:usize = 0;
        while !done {
            let start = Instant::now();
            let wf = WaveFunction { alpha: alphas[i], beta: betas[i], omega: OMEGA, jastrow_on: JASTROW }; // Set beta = gamma
            let mut system: System<N> = System::new(N, DIM, wf, interacting, numerical_laplace, SPREAD).unwrap();
            let vals = montecarlo::monte_carlo(MC_CYCLES, &mut system, &mut metro).unwrap();

            let energy = match vals.map.get("energy") {
                Some(val) => *val,
                None => 0.,
            };
            let energy_sqrd = match vals.map.get("energy_sqrd") {
                Some(val) => *val,
                None => 0.,
            };
            let wf_deriv_alpha = match vals.map.get("wf_deriv_alpha") {
                Some(val) => *val,
                None => 0.,
            };
            let wf_deriv_alpha_times_energy = match vals.map.get("wf_deriv_alpha_times_energy") {
                Some(val) => *val,
                None => 0.,
            };
            let wf_deriv_beta = match vals.map.get("wf_deriv_beta") {
                Some(val) => *val,
                None => 0.,
            };
            let wf_deriv_beta_times_energy = match vals.map.get("wf_deriv_beta_times_energy") {
                Some(val) => *val,
                None => 0.,
            };

            let data = format!("{},{},{},{},{}\n",alphas[i], betas[i], energy / N as f64, start.elapsed().as_millis() as f64 / 1000., energy_sqrd - energy.powi(2));
            //println!("{}", data);
            f.write_all(data.as_bytes()).expect("Unable to write data");
            println!("a: {:.8} || b: {:.8} || E: {:.8} || Iter: {}", alphas[i], betas[i], energy / N as f64, i);


            let energy_deriv_alpha = 2.* (wf_deriv_alpha_times_energy-wf_deriv_alpha*energy);
            let new_alpha: f64 = alphas[i];// - learning_rate * energy_deriv_alpha;
            alphas.push(new_alpha);

            let energy_deriv_beta = 2.* (wf_deriv_beta_times_energy-wf_deriv_beta*energy);
            let new_beta: f64 = betas[i] - learning_rate * energy_deriv_beta;
            betas.push(new_beta);

            if energy_deriv_alpha.abs() < TOLERANCE && energy_deriv_beta.abs() < TOLERANCE {
                println!("Tolerance is met, exiting.");
                done = true;
            } else if i > 500 {
                println!("Max iter lim met, exiting.");
                done = true;
            }
            //if (energies[i]-energies[i-1]).abs() < tolerance {
            //    done = true;
            //}
            i += 1;
        }
        
    }
    let start = Instant::now();
    //simulate::<ImportanceMetropolis>(1.0 ,1. , 0.1, true, interacting);

    
    // Multithreading
    println!("Running simulations using BruteForceMetropolis algorithm...");
    let start_alphas:Vec<f64> = vec![0.4, 0.6, 0.8, 1.0, 1.2, 1.4, 1.6, 1.8];
    let start_betas: Vec<f64> = vec![0.4, 0.6, 0.8, 1.0, 1.2, 1.4, 1.6, 1.8];
    let learning_rates:Vec<f64> = vec![0.00005, 0.0001, 0.0002, 0.0004, 0.0008, 0.0016, 0.0032, 0.0064];
    let start_alpha: f64 = 0.0;
    let start_beta: f64 = 0.0;
    let learning_rate: f64 = 0.02; //0.0004 was the chosen one for project 1

    println!("Spawning threadpool of 8 threads, with {} Monte Carlo cycles on each", &MC_CYCLES);
    

    
    for start_alpha in start_alphas {
    //for learning_rate in learning_rates {
        let pool = ThreadPool::new(8);
        let start = Instant::now();

        for start_beta in start_betas.clone() {
            pool.execute(move || simulate::<ImportanceMetropolis>(start_alpha, start_beta, learning_rate, true, interacting)); //Running the simulation on each thread individually
        }
        println!("All threads now executing with different betas and alpha = {} , waiting for them to finish...", &start_alpha);
        pool.join_all();
        println!("Time spent on all betas for alpha = {}: {:?}", &start_alpha, start.elapsed());
    }
    
    
    println!("Total time spent: {:?}", start.elapsed());
}



fn find_cargo_root() -> Option<PathBuf> {
    let mut path: PathBuf = env::current_dir().unwrap().into();
    let file = Path::new("Cargo.toml");

    loop {
        path.push(file);

        if path.is_file() {
            path.pop();
            break Some(path);
        }

        if !(path.pop() && path.pop()) {
            break None;
        }
    }
}

fn create_dir(path: &PathBuf) {
    if Path::new(path).exists() == false {
        create_dir_all(path).expect("Unable to create folder");
    }
}

fn create_file(filepath: &PathBuf) -> File {
    match File::create(filepath) {
        Ok(f) => f,
        Err(why) => panic!("Unable to create {:?}: {}", filepath, why),
    }
}
