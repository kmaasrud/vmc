# Variational Monte Carlo solver

The solver is written in Rust (source code in `src`), the data it produces is in `data` and the statistical analysis as well as plotting is written in Python (source code in `result_analysis`). 

## Solver explanation

`wavefunction.rs`, `hamiltonian.rs` and `particle.rs` all contain a similarly named structs representing their respective aspect of the system. They are all tied together in the `System` struct located in `system.rs`. These represent the system state and hold the equations to find relevant quantities.

Building on this, we've got the `Metropolis` trait located inside `metropolis.rs`, which describes an interface that is able to produce a Metropolis step. This trait is realised in the two structs `BruteForceMetropolis` and `ImportanceMetropolis`. Finally, located in `montecarlo.rs`, our Monte Carlo solver leverages this trait to perform an integration over the desired quantities. `hermite.rs` defines the Hermite polynomials, `vector.rs` holds the definition of a custom vector struct we are using to increase performance, and `utils.rs` hold a few handy functions used throughout the program.

The above are used to produce our results in the `run.rs` file. Here we've defined a collection of functions that produce different types of outputs to suit our needs. Also worth mentioning is the `threadpool.rs` file, which simplifies parallelization.
