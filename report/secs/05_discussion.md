# Discussion

## Verification/ Two electrons

As a validation test for our algorithm, we calculated the energy without the Jastrow factor for $N = 2$ electrons, expecting a energy of $2.0$ a.u. and a variance of zero. Running this system with the brute force Metropolis algorithm (as well as with importance sampling), returned an energy of $1.999$ ($1.999$) and a variance of $0$ ($0$). Together with the energy and statistical calculations, the performance analysis showed only a small difference in time consumed by the analytical and numerical approach. The analytical algorithm used $5.3$ ($6.1$) seconds, while the numerical approach used $5.8$ ($6.3$) seconds. Comparing the two sampling methods in question, the Metropolis sampling was faster than the Importance sampling method for this particular system. 

As shown in our previous work [@Vmc-bosonic2021], importance sampling generally used more time, and this is once again shown here.

<!-- The effect of blocking on the results Metropolis vs Importance-->


## Variational parameters
<!-- Two particle system-->

The variational parameters were obtained using the steepest gradient decent method with the Jastrow factor. The results for the two electron system are to be found in Table [@tbl:results-variational-parameters-2N]. With the Jastrow factor, both $\alpha$ and $\beta$ are to be variated during the same simulation. This has led to immense debugging as the beta value simply will not converge, except when it hits zero. We have gone over the code for days without finding what can be the cause of this, and made the tough decision that we could not spend more time on the issue. $\alpha$ still seems to converge, however because the energy from the monte carlo solver is not accurate enough, the SGD seems to have some issues reaching the lowest energy state.

Before doing the big run, different learning rates for the SGD was performed, and we saw no stability gains when having lower learning rates and more iterations in the SGD. Again, we believe that this ties back to the accuracy of the MC cycling, and the fact that something regarding the $\beta$ variable is malfunctioning.

<!-- Maybe write something about the influence of the step size, how it influences the steepest decent method performance - smaller step size = higher accuracy and more likely to hit the lowest energy, while a higher step size gives a higher performance/uses less time, but is less likely to hit bottom of the energy -->


<!--Higher number of particles: Comment if something was done differently compared to two particle system - e.g., using grid of alphas/betas instead of steepest gradient descent-->


## Minimum energy and particle distance
<!-- Two particle system-->

<!-- Compare values to Taut's article. E.g. for omega = 1, the energy should be 3 au. Then maybe give a deviation \% from Taut’s (2 omega). Also compare with and without the Jastrow factor and with and without interaction (Hamiltonian)-->

The results from the energy minima calculations using the optimal variational parameters are also listed in Table [@tbl:results-min-energy-particle-distance-2N-with] and [@tbl:results-min-energy-particle-distance-2N-without], for calculations with and without the Jastrow factor, respectivly. For the largest frequency, $\omega = 1$ we can from Taut's article [@Taut1993] expect an energy of 3 au when computing both with the Jastrow factor and with particle interaction. Our results in this mentioned case shows an energy of 3.07, which is fairly close. 

The obtained mean distance between two fermions shows to be strongly dependent on the frequency, $\omega$, as shown in Table [@tbl:results-min-energy-particle-distance-2N-with] and [@tbl:results-min-energy-particle-distance-2N-without]. A decrease in frequency of a factor 100 increases the distance with a factor of approximatly 10. Higher frequencies induces higher energies and particles then tends to be closer togheter. Studying the effect of the Jastrow factor, which are shown in the abovemention tables, it is clearly that the Jastrow factor keeps the particles further apart. 

The same dependecies are naturally reflected in the kinetic and potential energies calculated for different frequencies as well - which are listed in Table [@tbl:freq-dep-energies-2N]. The Jastrow factor, in general, gives a calculated energy closer to the one expected from Taut's article [@Taut1993]. Calculating the energy for two electrons with interaction and without the Jastrow factor gives an energy of $3.24$, while adding the Jastrow factor gives an energy of $3.07$. Hence, one sees the importance of adding the factor for a more accurate result. 


## One Body density{#sec:one-body-dens-discussion}

The results from the one body density calculations utilizing optimal parameters are shown in Figure [@fig:one-body-densities]. For both the calculations with and without the Jastrow factor, there is a peak density at $|\mathbf{r}| \approx 1$, which hence is where the particles are most likely located. For $|\mathbf{r}|$ approximatly less  than one,  the calculations without the Jastrow factor has a higher density, while for larger $|\mathbf{r}|$, the density is greater when the Jastrow factor is on. 

One-body density is not calculated for systems with larger number of particles due to reasons mentioned in earlier sections.  

## Frequency dependent energy calculations

As seen in table [@tbl:freq-dep-energies-2N], the expecation value of the kinetic and potential energy increases with increasing frequency , $\omega$. This is expected, as a harmonic oscillator has an stepwise increase in energy of $\frac{1}{2} \hbar \omega^2$  <!-- How does it behvave compared to the Virial theorem?-->

According to the Virial theorem for a Harmonic occilator, the mean potential energy, should equal the mean kinetic energy, which is however not the case. Indipendent of the frequencies, the potential energy is larger than the kinetic energy. 


## Larger systems and bugs in the code

We experienced a lot of trouble with getting the larger systems ($N > 2$) to work. The issue was that the Greens factor evaluated to $0$ with every step, leaving us with no results to do anything with. We suspect the issue is with our evaluation of the Slater gradient, and subsequently with how our quantum force is found, in order to do a step. We were not, however, able to locate this issue in due time, which left us in the awkward situation of not having anything to present.

## Performance analysis

Table [@tbl:results-performance-analysis] gives an overview of the performance analysis of running our algorithm with and without vectorization and parallelization for $N = 2$ electrons. As seen, there is a immense speedup utilizing vectorization. This results shows the importance of utilizing the tools available, especially for increased sized systems. 

Our code is parallelized in a way where different experiments are ran at seperate cores. This leads to a drastic speedup for our use case (simulating many different systems simultaneously), but does not qualify as a proper parallellization of the VMC method. Therefore, a seperate performance analysis on the performance increase parallelization is not conducted, as this obviously scales with the number of threads you run simulations in.


