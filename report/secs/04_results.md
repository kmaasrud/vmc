# Results

## Performance evaluation of different energy calculation methods{#sec:results-performance-calc-methods}

The performance of the analytical expression for the local energy, numerical derivation of the kinetic energy and the analytical local energy with importance sampling is compared in table [@tab:results-performance-calc-methods] below.

| **Calculation method**            | **Time spent (s)**    |
| ----:                             | ---                   |
| Analytical                        | $time$                |
| Numerical                         | $time$                |
| Analytical w/Importance Sampling  | $time$                |{#tab:results-performance-calc-methods} 

The blocking analysis shows that the optimal standard deviation is $FILL$.

## Evaluating the variational parameters{#sec:results-variational-params}

The VMC approximation to the correct energy dependent on the variational parameters $\alpha$ and $\beta$ are shown in the figures below.

## Computations for the two electron system

By using the variational parameters $\alpha = FILL ME$ and $\beta = FILL ME$, the calculation of minimum energy for the system has been compared to Taut's analytical values in table [@tab:results-calculations]. This table also contains the mean distance between the two electrons and the onebody density. Calculations are done with interaction, purely Harmonic Oscillator wavefunctions and pure HO wavefunctions without the Jastrow factor.

| **Calculation type** | **Minimum energy**| **Taut's analytical energies**| **Mean particle distance** | **Onebody Density**|
|---    | ---: | :--- | :---: | :---: |
| All included | $FILL$ | $2\omega$ |    $1$  |   $1$  |
| Purely HO | $FILL$ | $2\omega$ |    $1$  |   $1$  |
| Purely HO, no Jastrow | $FILL$ | $2\omega$ |    $1$  |   $1$  |{#tab:results-calculations}

Lastly the expectation value for the kinetic energy and the potential energy is computed using $\omega \in {0.01, 0.05, 0.1, 0.5, 1.0}$. With interaction and the Jastrow factor.

| **$\omega$**| **$\langle E_k \rangle$**| **$\langle E_p \rangle$** |
| --- | :---: | :---: |
| $0.01$    | $1$   |    $1$  |
| $0.05$    | $1$   |    $1$  |
| $0.1$     | $1$   |    $1$  |
| $0.5$     | $1$   |    $1$  |
| $1.0$     | $1$   |    $1$  |{#tab:results-calculations-omega}

