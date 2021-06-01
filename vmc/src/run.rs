use crate::{
    montecarlo,
    ThreadPool,
    System,
    BruteForceMetropolis,
    ImportanceMetropolis,
    WaveFunction,
    Hamiltonian,
    Metropolis,
};

use std::{
    env,
    time::Instant,
    fs::{File, create_dir_all},
    path::{Path, PathBuf},
    io::prelude::*,
    
};


#[allow(dead_code)]
pub fn simple(){
    //The run::simple() only does BruteForceMetropolis sampling without multithreading.
    
    const CSV_HEADER: &str = "Alpha,Energy,Time\n";
    const STEP_SIZE: f64 = 0.1;
    const NON_INTERACTING: bool = true;
    const MC_CYCLES: usize = 1_000_000;
    const DIM : usize = 2;
    const N: usize = 20;
    const INTERACT: bool = false;
    const SPREAD: f64 = 0.1;
    
    
    fn simulate<T: Metropolis>(){
        let alphas = [0.2, 0.25, 0.3, 0.35, 0.4, 0.45, 0.5, 0.55, 0.6, 0.65, 0.7, 0.75, 0.8];

        //let alphas : Vec<f64> = (3..190).map(|x| x as f64 / 100.).collect();
        
        let mut metro: T = T::new(STEP_SIZE);

        let mut path = find_cargo_root().unwrap();
        path.push("data"); path.push("ana_vs_num"); path.push(format!("{}D_{}N", (DIM as f64),N));
        create_dir(&path);
            
        path.push(format!("{}.csv", std::any::type_name::<T>().split("::").last().unwrap()));
        let mut f = create_file(&path);
        f.write_all(CSV_HEADER.as_bytes()).expect("Unable to write data");

        println!("Dimension: {}", DIM);
        
        for alpha in alphas.iter(){
            let start = Instant::now();
            let wf = WaveFunction{ alpha: *alpha, beta: 1., omega: 1. }; // Set beta = gamma
            let mut system: System<N> = System::new(N, DIM, wf, INTERACT, SPREAD).unwrap();
            let vals = montecarlo::monte_carlo(N, &mut system, &mut metro).unwrap();
            
            let energy = match vals.map.get("energy") {
                Some(val) => *val,
                None => 0.,
            };

            let data = format!("{},{},{:?}\n", alpha, energy, start.elapsed());
            println!("{}", data);
            f.write_all(data.as_bytes()).expect("Unable to write data");
            // println!("Time spent for alpha = {}: {:?}", alpha,  start.elapsed());
        }
            
    }   

    let start = Instant::now();
    //let pool = ThreadPool::new(2);
    //pool.execute(move || simulate::<BruteForceMetropolis>());
    //pool.execute(move || simulate::<ImportanceMetropolis>());
    //pool.join_all();
    simulate::<BruteForceMetropolis>();
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
