# Results

## Two electrons

To validate our algorithm a simulation of the simplest case with two electrons without the Jastrow factor and perturbation was done, expecting an energy-output of excactly 2 a.u and a variance of 0. The results, togheter with a perfomance analysis(see below), is listed in Table [@tbl:results-performance-calc-methods]. 

A performance analysis, taking the avarage time over several runs, of the analytical expression for the local energy, numerical derivation of the kinetic energy and the analytical local energy with importance sampling is compared in table[@tbl:results-performance-calc-methods] below.

| **Sampling method**               | **Avarage time [s]** | $\langle E \rangle$|$\langle E \rangle_{kinetic}$  |$\sigma^2$ |
| ----:                             | ---                  |---                 |---                            |---        |
| Analytical w/ Metropolis          | $time$               |                    |                               |           |
| Analytical w/ Importance Sampling | $time$               |                    |                               |           |
| Numerical  w/ Metrpolis           | $time$               |                    |                               |           |
| Numerical  w/ Importance Sampling | $time$               |                    |                               |           |   {#tbl:results-performance-calc-methods} 

The blocking analysis shows that the optimal standard deviation is $FILL$.

## Evaluating the variational parameters{#sec:results-variational-params}

The VMC approximation to the correct energy dependent on the variational parameters $\alpha$ and $\beta$ are shown in Table [@tbl:results-variational-parameters] below.


| N     	| $\omega$ 	| $\alpha$ 	| $\beta$ 	| $\alpha$ withouth Jastrow 	|
|-------	|:---------:|----------	|---------	|---------------------------	|
| 2     	| 1        	|          	|         	|                           	|
|       	| 0.5      	|          	|         	|                           	|
|       	| 0.1      	|          	|         	|                           	|
|       	| 0.05     	|          	|         	|                           	|
|       	| 0.01     	|          	|         	|                           	|
| ----- 	| -------- 	| -------- 	| ------- 	| --------------------------- 	|
| 6     	| 1        	|          	|         	|                           	|
|       	|  0.5     	|          	|         	|                           	|
|       	| 0.1      	|          	|         	|                           	|
|       	| 0.05     	|          	|         	|                           	|
|       	| 0.01     	|          	|         	|                           	|
| ----- 	| -------- 	| -------- 	| ------- 	| ---------------------------   |
| 12    	| 1        	|          	|         	|                           	|
|       	| 0.5      	|          	|         	|                           	|
|       	| 0.1      	|          	|         	|                           	|
|       	| 0.05     	|          	|         	|                           	|
|       	| 0.01     	|          	|         	|                           	|
| ----- 	| -------- 	| -------- 	| ------- 	| --------------------------- 	|
| 20    	| 1        	|          	|         	|                           	|
|       	| 0.5      	|          	|         	|                           	|
|       	| 0.1      	|          	|         	|                           	|
|       	| 0.05     	|          	|         	|                           	|
|       	| 0.01     	|          	|         	|                           	|

Table: Optimal variatonal parameters for the different systems with and without the Jastrow factor, for comparison.{#tbl:results-variational-parameters} 
## Computations for the two electron system

By using the variational parameters $\alpha = FILL ME$ and $\beta = FILL ME$, the calculation of minimum energy for the system has been compared to Taut's analytical values in table [@tbl:results-calculations]. This table also contains the mean distance between the two electrons and the onebody density. Calculations are done with interaction, purely Harmonic Oscillator wavefunctions and pure HO wavefunctions without the Jastrow factor.

| **Calculation type** | **Minimum energy**| **Taut's analytical energies**| **Mean particle distance** | **Onebody Density**|
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

## Performance analysis
<!--  -->
Lastly a analysis of the algorithms are given for $N = 6$ electrons, $FILL IN$ Monte Carlo cycles, optimal variational parameters  $\omega = 1$ and without the Jastrow factor. The analysis is done by comparing the avarage time used for a calculation with and without vecotrization. The procedure is repeated with paralellization, expecting approximatly a 100% speedup.  The quantities which are calculated are **FILL INN**. The most time consuming part is the **FILL INN** , and hence the clock is started here. The results are presented in table [@tbl:results-performence-analysis]. **ADD SOME MORE DESCRIPTION OF HOW HERE**

Table: Results from performance analysis with and without vectorization and compile flags. The time is avaraged over 10 runs with **FILL INN** MC cycles. The sampling method is the **Brute Force Metropolis OR Importance sampling**
| Optimization/compile flags 	| $\bar{t}$ [s] 	|
|----------------------------	|---------------	|
| **With vectorization**:       |               	|
|  Flag 1                       |                   |
| Flag 2                        |                   |
| Flag 2                        |                   |
| **Without Vectorization**:  	|               	|
| Flag 1                        |                   |
| Flag 2                        |                   |
| Flag 3                        |                   |
| Parallelization            	|               	|{#tbl:results-performence-analysis}





<!-- Nececerry to write something about which computers/specs the analysis is done at?? -->
