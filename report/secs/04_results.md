# Results

## Two fermions {#sec:two-fermions}

To validate our algorithm a simulation of the simplest case with two electrons without the Jastrow factor and perturbation was done, expecting an energy-output of exactly 2 au and a variance of 0. The results, together with a performance analysis (see below), is listed in Table [@tbl:results-performance-calc-methods]. 

A performance analysis, taking the average time over 10 runs, of the analytical expression for the local energy, numerical derivation of the kinetic energy and the analytical local energy with importance sampling is shown in table[@tbl:results-performance-calc-methods] below.


| Sampling method                   | Avarage time [s]|$\langle E \rangle$|$\langle E_{kinetic}\rangle$|$\sigma^2$|$\sigma_{\text{blocking}}$|
| ----:                             | ---             |---                |---                          |---       |---                      |
| Analytical w/ Metropolis          | 5.25            | 2.000             |1.03                         | 0.0000   | 0.0000                  |
| Analytical w/ Importance Sampling | 6.13            | 2.000             |1.04                         | 0.0000   | 0.0000                  |
| Numerical w/ Metropolis           | 5.82            | 1.9999            | 0.99                        | 0.0000   | 0.0000                  |
| Numerical w/ Importance Sampling  | 6.3             | 1.9999            | 1.02                        | 0.0000   | 0.0000                  |

Table: Results from computations of the expectation value of the energy using both Importance Sampling and the Metropolis algorithm for both the analytical expression for the local energy(see equation @eq:analytical) and numerical derivation of the kinetic energy. To compare the performance of the different configurations, the algorithms are timed over 10 runs and averaged. Statistical results from a blocking and a variance analysis is listed in the column $\sigma_\text{blocking}$ and $\sigma$, respectivly. {#tbl:results-performance-calc-methods} 


### Evaluating the variational parameters{#sec:results-variational-params}

To obtain the optimal variational parameters for the ground state energy, the steepest gradient decent method is implemented in the Variational Monte Carlo calculations. The result are shown in Table [@tbl:results-variational-parameters-2N].

| $\omega$ 	| $\alpha$ 	| $\beta$ 	| Energy [au] 	|
|----------	|----------	|---------	|------------	|
| $1.0$    	|          	|         	|            	|
| $0.5$    	|          	|         	|            	|
| $0.1$    	|          	|         	|            	|
| $0.05$   	|          	|         	|            	|
| $0.01$   	|          	|         	|            	|

Table: Optimal variational parameters for $N = 2$ electrons obtained with the steepest gradient decent method. {#tbl:results-variational-parameters-2N}


### Minimum energy and particle distance

| $\omega$ 	| $E_{\text{min}}$ 	| $\sigma_{\text{blocking}}$  |$E_{\text{kinetic}}$	| $r_{12}$ 	| Acceptance ratio 	|
|----------	|------------------	|---------------------------- | ------------------  |----------	|------------------	|
| $1.0$    	|   3.05           	|   1.509                     |	                    |          	|     0.979        	|
| $0.5$    	|   1.72           	|   0.551                     |	                    |          	|     0.985        	|
| $0.1$    	|   0.48          	|   0.057                     |	                    |          	|     0.992        	|
| $0.05$   	|   0.31           	|   0.012                     | 	                |          	|     0.995        	|
| $0.01$   	|   0.17           	|   0.031                     |  	                |          	|     0.998       	|

Table: Energy minimum computed and avarage particle distance computed for $N = 2$ electrons *with* the Jastrow factor. All enteties are calculated with the optimal set of variational parameters, see table [@tbl:results-variational-parameters-2N].{#tbl:results-min-energy-particle-distance-2N}


| $\omega$ 	| $E_{\text{min, wo/Jastrow}}$ 	| $\sigma_{\text{blocking}}$  | $E_{\text{kinetic}}$    | $r_{12, wo/Jastrow}$ 	| Acceptance ratio 	|
|----------	|------------------------------	|---------------------------- | ----------------------  |----------------------	|------------------	|
| $1.0$    	|  3.24                       	|   10.1                      |	0.983                   | 1.267                	|  0.892          	|
| $0.5$    	|  1.86                        	|   2.433                     |	0.502                   | 1.768                	|  0.925           	|
| $0.1$    	|  0.617                      	|   0.602                     | 0.102                   | 3.943                	|  0.967           	|
| $0.05$   	|  0.400                       	|   0.447                     | 0.053                   | 5.366                	|  0.977           	|
| $0.01$   	|  0.148                       	|   0.058                     | 0.010                   | 12.365               	|  0.990           	|

Table: Energy minimum computed and avarage particle distance computed for $N = 2$ electrons *without* the Jastrow factor. All enteties are calculated with the optimal set of variational parameters, see table [@tbl:results-variational-parameters-2N].{#tbl:results-min-energy-particle-distance-2N}

### One-body density

The One-body density for the two electron system with optimal set of variational parameters with and without the Jastrow factor is shown in Figure [@fig:one-body-densities-2N]

![One Body densities for 2 fermions with and without the Jastrow factor. The computations are done with $\alpha = 0.98$, $\beta = 0.43$ and $\omega = 1$](onebodydens-2N.png){#fig:one-body-densities-2N width=300px}

### Frequency dependency

The expectation value of the kinetic and potential energy using a set of frequencies are listed in Table [@tbl:freq-dep-energies-2N]

| $\omega$ 	| $E_{\text{kinetic}}$ 	| $E_{\text{potential}}$ 	|
|----------	|----------------------	|------------------------	|
| $1.0$    	|                      	|                        	|
| $0.5$    	|                      	|                        	|
| $0.1$    	|                      	|                        	|
| $0.05$   	|                      	|                        	|
| $0.01$   	|                      	|                        	|

Table: Frequency dependent calculations of the kinetic and potential energies. {#tbl:freq-dep-energies-2N}

## Larger systems

Our code has a bug that leads to the steps in systems with $N > 2$ not being accepted. This leads to us not having any results to show for said systems. We were not able to fix this in time for the delivery.

## Performance analysis

Lastly a analysis of the algorithms are given for $N = 2$ electrons, 1 000 000 Monte Carlo cycles, optimal variational parameters $\omega = 1$ and with the Jastrow factor included. The analysis is done by comparing the average time used for a calculation with and without vectorization. All quantities were calculated, and no writing to file was done (we timed only the Monte Carlo integration). The results are presented in table [@tbl:results-performance-analysis].

| Optimization               	| $t$ [s] |
|---                        	|---            |
| **Without vectorization**		| 39.61s |
| **With vectorization**        | 1.78s  |

Table: Results from performance analysis with and without vectorization. The time is averaged over 10 runs with 1 000 000 MC cycles. The sampling method used is the brute force Metropolis method. {#tbl:results-performance-analysis}



<!-- Necessary to write something about which computers/specs the analysis is done at?? -->
