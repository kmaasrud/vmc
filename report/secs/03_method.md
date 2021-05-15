# Method

## Variational Monte Carlo

Our variational Monte Carlo approach is as explained by our previous work [@Vmc-bosonic2021]. Roughly, it proceeds by proposing a change to the system $\mathbf R \mapsto \mathbf R'$ by changing the position of a single particle $\mathbf r_i$. The choice of this particle and how it moves is done both randomly and by way of the *quantum force*, both explained in @Vmc-bosonic2021. From the states $\mathbf R$ and $\mathbf R'$, and the trial wave function $\Psi_T$, we evaluate an acceptance factor, that determines whether or not we accept the proposed changed system.

Regardless of whether the new step is accepted or not, the desired quantities - in our case the energy, it's gradient and their composites - are sampled in Monte Carlo integration. The integrated values are then used in steepest gradient descent to find the optimal variational parameters.

## Testing

Testing in Rust is normally divided in two categories: *unit tests* and *integration tests*. Unit tests are small codes to test specific functions inside the code. These tests are normally written in the same file as the functions themselves, but inside a module annotated with `cfg(test)`.

On the other hand, integration tests are written externally to the library, and is made to test the integration of the functions in the program. These tests are often much larger than unit tests, and are made to make sure that the internal functions work well with each other, from the standpoint of an external user. Therefore, integration tests are normally written in a separate `tests` directory at the same level as the `src` directory.

We will write mainly unit tests in our program, to ensure that our functions return the expected values, and to reduce the mental overhead of debugging when making larger changes to the codebase.

More on testing can be found in the official documentation of the Rust programming language [@Rust-docs-testing].

## Parallelization

![Temporary diagram for visualization](diagrams/metropolis-hastings-tree.jpg)
