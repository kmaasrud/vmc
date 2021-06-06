# Abstract

Rust is used to develop a ground state solver for fermionic systems in a harmonic oscillator potential, utilizing the variational principle, Monte Carlo integration and two different implementations of the Metropolis-Hastings algorithm. The results are compared with analytical solutions to the corresponding systems. 

The effect of the so-called Jastrow factor shows overall a decrease in energies. In the case of the one-body densities, there is a decrease in density for lower $|\mathbf{r}$, while for $|\mathbf{r} > 1$ approximately, the density is higher for calculations with the Jastrow factor. Performance analysis shows a 20 times speedup using vectorization. 

Our program did not manage to produce results for larger systems because of an unknown bug introduced in our code at a late stage. We were not able to locate it in time for the deadline, and are thus lacking some expected results. Properly removing this bug and getting a fully functional solver is high on our priority list for future work.
