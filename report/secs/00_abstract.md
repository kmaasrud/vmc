# Abstract

Rust is used to develop a ground state solver for fermionic systems in a harmonic oscillator potential, utilizing the variational principle, Monte Carlo integration and two different implementations of the Metropolis-Hastings algorithm. The results are compared with analytical solutions to the corresponding systems. 

<!-- Highlight some of the results quantitatively - eg the effect of the Jastrow factor --> 
The effect of the so-called Jastrow factor shows overall a decrease on energies. In the case of one-body densities, there is a decrease in density for lower $|\mathbf{r}$, while for $|\mathbf{r} > 1$ approximately, the density is higher for calculations with the Jastrow factor. Performance analysis shows a 20 times speedup using vectorization. 