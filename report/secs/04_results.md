# Results

## Two fermions

To validate our algorithm a simulation of the simplest case with two electrons without the Jastrow factor and perturbation was done, expecting an energy-output of exactly 2 au and a variance of 0. The results, together with a performance analysis (see below), is listed in Table [@tbl:results-performance-calc-methods]. 

A performance analysis, taking the average time over 10 runs, of the analytical expression for the local energy, numerical derivation of the kinetic energy and the analytical local energy with importance sampling is shown in table[@tbl:results-performance-calc-methods] below.


| Sampling method                   | Avarage time [s]     | $\langle E \rangle$|$\langle E_{kinetic} \rangle$  |$\sigma^2$ |$\sigma_{\text{blocking}}$ |
| ----:                             | ---                  |---                 |---                            |---        |---                        |
| Analytical w/ Metropolis          | $time$               |                    |                               |           |                           |
| Analytical w/ Importance Sampling | $time$               |                    |                               |           |                           |
| Numerical w/ Metropolis           | $time$               |                    |                               |           |                           |
| Numerical w/ Importance Sampling | $time$               |                    |                               |           |                           |
Table: Results from computations of the expectation value of the energy using both Importance Sampling and the Metropolis algorithm for both the analytical expression for the local energy(see equation @eq:analytical) and numerical derivation of the kinetic energy. To compare the performance of the different configurations, the algorithms are timed over 10 runs and averaged.   {#tbl:results-performance-calc-methods} 

The **blocking analysis** shows that the optimal standard deviation is $FILL$.

## Evaluating the variational parameters{#sec:results-variational-params}

To obtain the optimal variational parameters for the ground state energy, the steepest decent method is implemented in the Variational Monte Carlo calculations. The result, with and without the Jastrow factor are shown in Table [@tbl:results-variational-parameters]. The table also 


| N   | $\omega$ | $\alpha$ | $\alpha_{\text{wo J}}$ | $\beta$ | $\langle E \rangle$ | $\sigma_{\text{blocking}}$ | $\langle E_k \rangle$ | $\langle E_p \rangle$ | $\langle r_{12}\rangle$ |
|-----|----------|----------|------------------------|---------|---------------------|----------------------------|-----------------------|-----------------------|-------------------------|
| 2   | 1        |          |                        |         |                     |                            |                       |                       |                         |
|     | 0.5      |          |                        |         |                     |                            |                       |                       |                         |
|     | 0.1      |          |                        |         |                     |                            |                       |                       |                         |
|     | 0.05     |          |                        |         |                     |                            |                       |                       |                         |
|     | 0.01     |          |                        |         |                     |                            |                       |                       |                         |
| --- | -------- | -------- | -------:               |         |                     |                            |                       |                       |                         |
| 6   | 1        |          |                        |         |                     |                            |                       |                       |                         |
|     | 0.5      |          |                        |         |                     |                            |                       |                       |                         |
|     | 0.1      |          |                        |         |                     |                            |                       |                       |                         |
|     | 0.05     |          |                        |         |                     |                            |                       |                       |                         |
|     | 0.01     |          |                        |         |                     |                            |                       |                       |                         |
| --- | -------- | -------- | -------                |         |                     |                            |                       |                       |                         |
| 12  | 1        |          |                        |         |                     |                            |                       |                       |                         |
|     | 0.5      |          |                        |         |                     |                            |                       |                       |                         |
|     | 0.1      |          |                        |         |                     |                            |                       |                       |                         |
|     | 0.05     |          |                        |         |                     |                            |                       |                       |                         |
|     | 0.01     |          |                        |         |                     |                            |                       |                       |                         |
| --- | -------- | -------- | -------                |         |                     |                            |                       |                       |                         |
| 20  | 1        |          |                        |         |                     |                            |                       |                       |                         |
|     | 0.5      |          |                        |         |                     |                            |                       |                       |                         |
|     | 0.1      |          |                        |         |                     |                            |                       |                       |                         |
|     | 0.05     |          |                        |         |                     |                            |                       |                       |                         |
|     | 0.01     |          |                        |         |                     |                            |                       |                       |                         |

Table: Optimal variational parameters for the different systems without the Jastrow factor, for comparison.{#tbl:results-variational-parameters} 



## Computations for the two-electron system

By using the variational parameters $\alpha = FILL ME$ and $\beta = FILL ME$, the calculation of minimum energy for the system has been compared to Taut's analytical values in table [@tbl:results-calculations]. This table also contains the mean distance between the two electrons and the one-body density. Calculations are done with interaction, purely Harmonic Oscillator wavefunctions and pure HO wavefunctions without the Jastrow factor.

| **Calculation type** | **Minimum energy**| **Taut's analytical energies**| **Mean particle distance** | **One-body Density**|
|---    | ---: | :--- | :---: | :---: |
| All included | $FILL$ | $2\omega$ |    $1$  |   $1$  |
| Purely HO | $FILL$ | $2\omega$ |    $1$  |   $1$  |
| Purely HO, no Jastrow | $FILL$ | $2\omega$ |    $1$  |   $1$  |{#tbl:results-calculations}

Lastly the expectation value for the kinetic energy and the potential energy is computed using $\omega \in {0.01, 0.05, 0.1, 0.5, 1.0}$. With interaction and the Jastrow factor.

| **$\omega$**| **$\langle E_k \rangle$**| **$\langle E_p \rangle$** |
| --- | :---: | :---: |
| $0.01$    | $1$   |    $1$  |
| $0.05$    | $1$   |    $1$  |
| $0.1$     | $1$   |    $1$  |
| $0.5$     | $1$   |    $1$  |
| $1.0$     | $1$   |    $1$  |{#tbl:results-calculations-omega}



## Ground state energies


## One body densities
The one body density is computed for 2 and 6 particles with the optimal parameters obtained during the previous calculations and $\omega = 1$. The calculations are executed with and without the Jastrow factor **(and perturbation)???** for comparison and analysis of their influence. 

The one-body densities are calculated with and without the Jastrow factor for two (2) and (6) fermions. The results are shown in figure [@fig:one-body-densities] below

![One Body densities for 2 and 6 fermions with and without the Jastrow factor. The computations are done with $\alpha = FILL$, $\beta = FILL$ and $\omega = 1$](FILENAME.png){#fig:one-body-densities width=300px}

## Performance analysis
<!--  -->
Lastly a analysis of the algorithms are given for $N = 6$ electrons, $FILL IN$ Monte Carlo cycles, optimal variational parameters $\omega = 1$ and without the Jastrow factor. The analysis is done by comparing the average time used for a calculation with and without vectorization. The procedure is repeated with parallelization, expecting approximately a 100% speedup.  The quantities which are calculated are **FILL INN**. The most time-consuming part is the **FILL INN**, and hence the clock is started here. The results are presented in table [@tbl:results-performence-analysis]. **ADD SOME MORE DESCRIPTION OF HOW HERE**

Table: Results from performance analysis with and without vectorization and compile flags. The time is averaged over 10 runs with **FILL INN** MC cycles. The sampling method is the **Brute Force Metropolis OR Importance sampling**

| Optimization/compile flags 	| $\bar{t}$ [s] |
|---                        	|---            |
| **With vectorization**        |               |
|  Flag 1                       |               |
| Flag 2                        |               |
| Flag 2                        |               |
| **Without Vectorization**  	|               |
| Flag 1                        |               |
| Flag 2                        |               |
| Flag 3                        |               |
| **Parallelization**           |               |{#tbl:results-performence-analysis}





<!-- Necessary to write something about which computers/specs the analysis is done at?? -->
