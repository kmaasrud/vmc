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
    const ALPHA: f64 = 2.0;
    const OMEGA: f64 = 1.0;
    const BETA: f64 =  0.0;
    const STEP_SIZE: f64 = 0.01;
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
        path.push(format!("{}_{}_{}.csv", metro_type, interact_str, numerical_str));
        let mut f = create_file(&path);
        f.write_all("energy[au],time[s],variance\n".as_bytes()).expect("Unable to write data");

        // Run 10 times
        for _ in 0..10 {
            let start = Instant::now();
            let wf = WaveFunction { alpha: ALPHA, beta: BETA, omega: OMEGA }; // Set beta = gamma
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

            let data = format!("{},{},{}\n", energy / N as f64, start.elapsed().as_millis() as f64 / 1000., energy_sqrd - energy.powi(2));
            println!("{}", data);
            f.write_all(data.as_bytes()).expect("Unable to write data");
        }
    }

    let start = Instant::now();
    // let pool = ThreadPool::new(8);
   /*  pool.execute(move || simulate::<BruteForceMetropolis>(false, false));
    pool.execute(move || simulate::<BruteForceMetropolis>(false, false));
    pool.execute(move || simulate::<BruteForceMetropolis>(false, false));
    pool.execute(move || simulate::<BruteForceMetropolis>(false, false));
    pool.execute(move || simulate::<ImportanceMetropolis>(false, false));
    pool.execute(move || simulate::<ImportanceMetropolis>(false, false));
    pool.execute(move || simulate::<ImportanceMetropolis>(false, false));
    pool.execute(move || simulate::<ImportanceMetropolis>(false, false));
    pool.join_all();  */
    simulate::<BruteForceMetropolis>(false, false);
    println!("Total time spent: {:?}", start.elapsed());
}

#[allow(dead_code)]
pub fn sgd(interacting: bool) {
    const ALPHA: f64 = 1.0;
    const OMEGA: f64 = 1.0;
    const BETA: f64 = 0.;
    const STEP_SIZE: f64 = 0.01;
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

        let mut metro: T = T::new(STEP_SIZE);

        let mut done: bool = false;
        let mut energies:Vec<f64> = vec![];

        let mut path = find_cargo_root().unwrap();
        //path.push("data"); path.push("sgd_noninteracting"); path.push("start-alpha");
        path.push("data"); path.push("sgd"); path.push("learning-rate");
        create_dir(&path);
        //path.push(format!("start-alpha_{}.csv", start_alpha));
        path.push(format!("learning-rate_{}.csv", learning_rate));
        let mut f = create_file(&path);
        f.write_all("alpha,energy-per-particle[au],time[s],variance\n".as_bytes()).expect("Unable to write data");

        let mut i:usize = 0;
        while !done {
            let start = Instant::now();
            let wf = WaveFunction { alpha: alphas[i], beta: start_beta, omega: OMEGA }; // Set beta = gamma
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
            let wf_deriv = match vals.map.get("wf_deriv_alpha") {
                Some(val) => *val,
                None => 0.,
            };
            let wf_deriv_times_energy = match vals.map.get("wf_deriv_alpha_times_energy") {
                Some(val) => *val,
                None => 0.,
            };

            let data = format!("{},{},{},{}\n",alphas[i], energy / N as f64, start.elapsed().as_millis() as f64 / 1000., energy_sqrd - energy.powi(2));
            println!("{}", data);
            f.write_all(data.as_bytes()).expect("Unable to write data");
            println!("Alpha: {:.16} --- Learning Rate: {:.2} --- Energy: {:.16} --- Iteration: {}", alphas[i], learning_rate, energy / N as f64, i);


            let energy_deriv = 2.* (wf_deriv_times_energy-wf_deriv*energy);
            let new_alpha: f64 = alphas[i] - learning_rate * energy_deriv;
            alphas.push(new_alpha);

            if energy_deriv.abs() < TOLERANCE {
                println!("Tolerance is met, exiting.");
                done = true;
            } else if i > 150 {
                println!("Max lim met, exiting.");
                done = true;
            }
            //if (energies[i]-energies[i-1]).abs() < tolerance {
            //    done = true;
            //}
            i += 1;
        }
        
    }
    let start = Instant::now();
    simulate::<BruteForceMetropolis>(0.2 ,1. , 0.1, false, interacting);

    /*
    // Multithreading
    println!("Running simulations using BruteForceMetropolis algorithm...");
    let start_alphas:Vec<f64> = vec![0.4, 0.6, 0.8, 1.0, 1.2, 1.4, 1.6, 1.8];
    let start_betas: Vec<f64> = vec![0.4, 0.6, 0.8, 1.0, 1.2, 1.4, 1.6, 1.8];
    let learning_rates:Vec<f64> = vec![0.00005, 0.0001, 0.0002, 0.0004, 0.0008, 0.0016, 0.0032, 0.0064];
    let start_alpha: f64 = 0.2;
    let start_beta: f64 = 0.0;
    let learning_rate: f64 = 0.0004;

    println!("Spawning threadpool of 8 threads, with {} Monte Carlo cycles on each", &MC_CYCLES);
    

    
    for start_alpha in start_alphas {
    //for learning_rate in learning_rates {
        let pool = ThreadPool::new(8);
        let start = Instant::now();

        for start_beta in start_betas {
            pool.execute(move || simulate(start_alpha, start_beta, learning_rate, false, interacting)); //Running the simulation on each thread individually
        }
        println!("All threads now executing with different betas and alpha = {.3}, waiting for them to finish...", &start_alpha);
        pool.join_all();
        println!("Time spent on all betas for alpha = {.4}: {:?}", &start_alpha, start.elapsed());
    }
    
    */
    println!("Time spent: {:?}", start.elapsed());
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
