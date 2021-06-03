mod hamiltonian;
mod hermite;
mod metropolis;
mod montecarlo;
mod particle;
mod run;
mod system;
mod threadpool;
mod utils;
mod vector;
mod wavefunction;

pub use hamiltonian::Hamiltonian;
pub use hermite::Hermite;
pub use metropolis::{BruteForceMetropolis, ImportanceMetropolis, Metropolis};
pub use montecarlo::monte_carlo;
pub use particle::Particle;
pub use system::System;
pub use threadpool::ThreadPool;
pub use utils::{Spin, a};
pub use vector::Vector;
pub use wavefunction::{WaveFunction, QUANTUM_NUMBERS};

fn main() {
    println!("Hello from VMC!");
    println!("Running run::simple()"); run::simple(); 
    // println!("Running run::sgd()"); run::sgd(false);
}
