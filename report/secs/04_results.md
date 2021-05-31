# Results

## Performance evaluation of different energy calculation methods{#sec:results-performance-calc-methods}

The performance of the analytical expression for the local energy, numerical derivation of the kinetic energy and the analytical local energy with importance sampling is compared in table [@tbl:results-performance-calc-methods] below.

| **Calculation method**            | **Time spent (s)**    |
| ----:                             | ---                   |
| Analytical                        | $time$                |
| Numerical                         | $time$                |
| Analytical w/Importance Sampling  | $time$                |{#tbl:results-performance-calc-methods} 

The blocking analysis shows that the optimal standard deviation is $FILL$.

## Evaluating the variational parameters{#sec:results-variational-params}

The VMC approximation to the correct energy dependent on the variational parameters $\alpha$ and $\beta$ are shown in Table [@tbl:results-variational-parameters] below.

Table: Optimal variatonal parameters for the different systems with and without the Jastrow factor, for comparison. 
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
|       	| 0.01     	|          	|         	|                           	|{#tbl:results-variational-parameters} 
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
Lastly a analysis of the algorithms are given for $N = 6$ electrons, $FILL IN$ Monte Carlo steps, optimal variational parameters, $\omega = 1$ and with the Jastrow factor off. The analysis is done by comparing the avarage time used for a calculation with and withouth vecotrization. The procedure is repeated with paralellization, expecting approximatly a 100% speedup.  The quantities which are calculated are the expectation energy, the kinetic and the potential energy. The most time consuming part is the **FILL INN** , and hence the clock is started here. 

<!-- Nececerry to write something about which computers/specs the analysis is done at?? -->
