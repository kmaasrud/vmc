# Method

## Variational Monte Carlo

Our variational Monte Carlo approach is as explained in our previous work [@Vmc-bosonic2021]. Roughly, it proceeds by proposing a change to the system $\mathbf R \mapsto \mathbf R'$ by changing the position of a single particle $\mathbf r_i$. The choice of this particle and how it moves is done both randomly and by way of the *quantum force*, both explained by @Vmc-bosonic2021. From the states $\mathbf R$ and $\mathbf R'$, and the trial wave function $\Psi_T$, we evaluate an acceptance factor, that determines whether or not we accept the proposed changed system. The flowchart shown in figure [@fig:fermion-vmc] briefly describes this process.

![Flowchart showcasing our Monte Carlo sampling](diagrams/fermion-vmc.png){#fig:fermion-vmc width=300px}

Regardless of whether the new step is accepted or not, the desired quantities - in our case the energy, its gradient and their composites - are sampled in Monte Carlo integration. The integrated values are then used in steepest gradient descent [@Vmc-bosonic2021] to find the optimal variational parameters.

\FloatBarrier

## Optimization of the wave function ratio

In our approach, the most time-consuming calculation is the evaluation of the wave function. For each proposed step in the Metropolis algorithm, we need to evaluate it to determine the acceptance factor, and if the step is accepted, yet another evaluation is needed (although this might be stored for reuse). This is expensive, so we need to optimize this process to scale well with the size of the system.

As previously stated, we find the acceptance factor in our Metropolis algorithm by introducing the proposed system change $\mathbf R \mapsto \mathbf R'$ in our Metropolis algorithm, with a single particle change $\mathbf r_p \mapsto \mathbf r_p'$. The acceptance factor depends on the wave function ratio $\mathcal R$, which we can split up like this:

$$ \mathcal R = \frac{\Psi_T(\mathbf R')}{\Psi_T(\mathbf R)} = \frac{\Psi_D(\mathbf R')}{\Psi_D(\mathbf R)}\cdot \frac{\Psi_J(\mathbf R')}{\Psi_J(\mathbf R)} = \mathcal R_D \mathcal R_J. $$

We will optimize each of these ratios separately.

### Optimizing $\mathcal R_D$

Each new ratio $\mathcal R_D = \frac{\det(D(\mathbf R'))}{\det(D(\mathbf R))}$ would require $\mathcal O(N^3)$ operations if done with Gaussian elimination. However, as found by @NukalaKent2009, this is decreased to $\mathcal O(N)$ by utilizing the inverse matrix $D^{-1}$ as such:

$$ \mathcal R_D = 1 + \mathbf v_p^T D^{-1}\mathbf e_p,\quad \text{where } \mathbf v_p = \begin{bmatrix}\phi_1(\mathbf r'_p) - \phi_1(\mathbf r_p) \\ \vdots \\ \phi_N(\mathbf r'_p) - \phi_N(\mathbf r_p)\end{bmatrix}. $$

$\mathbf e_p$ is simply the unit vector with $1$ on the $p$-th entry and zero everywhere else. It serves the purpose of extracting the $p$-th column from $D^{-1}$.

Now, how do we calculate $D^{-1}$ effectively? Once again, Gaussian elimination gives us an $\mathcal O(N^3)$ cost, which is no-go. However, if we do the matrix inversion with Gaussian elimination initially, to acquire $D_0^{-1}$, we can iteratively find the succeeding inversions by using a special case of the *Sherman-Morrison-Woodbury formula*[^smw] [@GolubLoan2013], which states:

$$ \left(D + D^{-1}\mathbf e_p \mathbf v_p^T\right)^{-1} = D^{-1} - \frac{D^{-1}\mathbf e_p \mathbf v_p^T D^{-1}}{1 + \mathbf v_p^T D^{-1}\mathbf e_p}. $$

We introduce the index $k$, referring to an arbitrary Monte Carlo step. Recognizing $D_k + D_k^{-1}\mathbf e_p \mathbf v_p^T$ as $D_{k+1}$ [@NukalaKent2009], we can simplify this to the iterative statement

$$ D_{k+1}^{-1} = \left(\mathbf I - \frac{D_k^{-1}\mathbf e_p \mathbf v_p^T}{\mathcal R_{D, k}}\right)D_k^{-1}. $$

This has an operation complexity of $\mathcal O(N)$.

[^smw]: Which, confusingly, is just called the *Sherman-Morrison formula*.

### Optimizing $\mathcal R_J$

We consider the ratio

$$ \mathcal R_J = \frac{\Psi'_J}{\Psi_J} = \prod_{i<j}^N \frac{\exp(J(r_{ij}', \beta))}{\exp (J(r_{ij}, \beta))}. $$

This naive operation scales in the order of $\mathcal O\left(\frac{N^2(N-1)}{2}\right)$. It is however easily optimized, by employing the fact that we only move one particle's position $\mathbf r_p$, which means only distances $r_{pj}$ are changed. We thus get

$$ \mathcal R_J = \prod_{i\ne p}^N \frac{\exp(J(r_{pi}', \beta))}{\exp(J(r_{pi}, \beta))} = \exp\left(\sum_{i \ne p}^N J(r_{pi}', \beta) - J(r_{pi}, \beta)\right), $$

which scales in the order of $\mathcal O(N-1)$ operations.


## Testing

Testing in Rust is normally divided in two categories: *unit tests* and *integration tests*. Unit tests are small codes to test specific functions inside the code. These tests are normally written in the same file as the functions themselves, but inside a module annotated with `#[cfg(test)]`.

On the other hand, integration tests are written externally to the library, and is made to test the integration of the functions in the program. These tests are often much larger than unit tests and are made to make sure that the internal functions work well with each other, from the standpoint of an external user. Therefore, integration tests are normally written in a separate `tests` directory at the same level as the `src` directory.

We will write mainly unit tests in our program, to ensure that our functions return the expected values, and to reduce the mental overhead of debugging when making larger changes to the codebase.

More on testing can be found in the official documentation of the Rust programming language [@Rust-docs-testing].

## Evaluation and performance of the VMC solver

The first performance evaluations are done for a case with two electrons in a quantum dot with frequency of $\hbar \omega = 1$.

###  Performance evaluation of different energy calculation methods

The performance of the analytical expression for the local energy is compared to the performance of the numerical derivation of the kinetic energy in results section [@sec:results-performance-calc-methods].  This test is performed without importance sampling and the Jastrow factor. Following this, importance sampling is added and tested only with the analytical expression for the local energy. Lastly, a blocking analysis is performed in order to obtain the optimal standard deviation.

The energy should equal 2.0 atomic units with a variance exactly equal to zero.

### Evaluating the variational parameters

By using the steepest descent method, the best variational parameters, $\alpha$ and $\beta$ are found. The results for this is found in section [@sec:results-variational-params].


### Computation of the two-electron system

The minimum energy of the system is computed and compared to Taut's work <!-- [@cite taut] -->. In addition, the mean distance between the two electrons and the one-body density is calculated for the best variational parameters. These results are also compared with the results form the same computations, where only the pure harmonic oscillator wavefunctions are used, and where pure HO wavefunctions are used but without the Jastrow factor.

Lastly the expectation value for the kinetic energy is calculated with $\omega \in {0.01, 0.05, 0.1, 0.5, 1.0}$.

## Increasing computational performance 

Being able to simulate many-body system at large scale without running out of time is crucial. Hence taking advantage of available tools such as compilation flags (e.g., for vectorization) and parallelization is important. The Rust language provides a great set of such tools - like the ones used in C++, but safer.

Since we run many different simulations with unique parameters, we parallelize over these simulations, to keep the logic of our program simple. This allows us to utilize all our cores' computation power, while still not needing to program concurrently. For future work, parallelization should be done further into the "core" of the program, to increase performance in single runs. For details on how we do vectorization in Rust, see @Vmc-bosonic2021.
