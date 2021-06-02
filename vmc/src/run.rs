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
    const STEP_SIZE: f64 = 1.0;
    const MC_CYCLES: usize = 100_000;
    const DIM: usize = 2;
    const N: usize = 2;
    const SPREAD: f64 = 0.1;
    const NUMERICAL_LAPLACE: bool = true;

    fn simulate<T: Metropolis>(numerical_laplace: bool, interacting: bool) {
        let mut metro: T = T::new(STEP_SIZE);

        let mut path = find_cargo_root().unwrap();
        path.push("data");
        path.push("N2");
        create_dir(&path);

        let metro_type = std::any::type_name::<T>().split("::").last().unwrap();
        let interact_str = if interacting { "interacting" } else { "non-interacting" };
        let numerical_str = if numerical_laplace { "numerical" } else { "analytical" };
        path.push(format!("{}_{}_{}.csv", metro_type, interact_str, numerical_str));
        let mut f = create_file(&path);
        f.write_all("energy-per-particle[au],time[s],variance\n".as_bytes()).expect("Unable to write data");

        // Run 10 times
        for _ in 0..10 {
            let start = Instant::now();
            let wf = WaveFunction { alpha: ALPHA, beta: 1., omega: OMEGA }; // Set beta = gamma
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
    let pool = ThreadPool::new(8);
    pool.execute(move || simulate::<BruteForceMetropolis>(false, false));
    pool.execute(move || simulate::<BruteForceMetropolis>(false, true));
    pool.execute(move || simulate::<BruteForceMetropolis>(true, false));
    pool.execute(move || simulate::<BruteForceMetropolis>(true, true));
    pool.execute(move || simulate::<ImportanceMetropolis>(false, false));
    pool.execute(move || simulate::<ImportanceMetropolis>(false, true));
    pool.execute(move || simulate::<ImportanceMetropolis>(true, false));
    pool.execute(move || simulate::<ImportanceMetropolis>(true, true));
    pool.join_all();
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
