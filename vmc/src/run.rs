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
    const STEP_SIZE: f64 = 1.0;
    const MC_CYCLES: usize = 10_000;
    const DIM: usize = 2;
    const N: usize = 2;
    const INTERACT: bool = false;
    const SPREAD: f64 = 0.1;

    fn simulate<T: Metropolis>(alpha: f64, omega: f64) {
        let mut metro: T = T::new(STEP_SIZE);

        let mut path = find_cargo_root().unwrap();
        path.push("data");
        path.push("N2");
        create_dir(&path);

        path.push(format!(
            "alpha_{}_{}.csv",
            alpha,
            std::any::type_name::<T>().split("::").last().unwrap()
        ));
        let mut f = create_file(&path);
        f.write_all("energy,time".as_bytes())
            .expect("Unable to write data");

        println!("Dimension: {}", DIM);

        let start = Instant::now();
        let wf = WaveFunction {
            alpha,
            beta: 1.,
            omega,
        }; // Set beta = gamma
        let mut system: System<N> = System::new(N, DIM, wf, INTERACT, SPREAD).unwrap();
        let vals = montecarlo::monte_carlo(MC_CYCLES, &mut system, &mut metro).unwrap();

        let energy = match vals.map.get("energy") {
            Some(val) => *val,
            None => 0.,
        };

        let data = format!("{},{:?}\n", energy, start.elapsed());
        println!("{}", data);
        f.write_all(data.as_bytes()).expect("Unable to write data");
    }

    let start = Instant::now();
    //let pool = ThreadPool::new(2);
    //pool.execute(move || simulate::<BruteForceMetropolis>());
    //pool.execute(move || simulate::<ImportanceMetropolis>());
    //pool.join_all();
    simulate::<BruteForceMetropolis>(1.0, 1.);
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
