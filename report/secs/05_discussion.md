# Discussion

## Verification/ Two electrons

As a validation test for our algorithm, we calculated the energy without the Jastrow factor for $N = 2$ electrons, expecting a energy of $2.0$ a.u. and a variance of zero. Running this system with the brute force Metropolis algorithm (as well as with importance sampling), returned an energy of $1.999$ ($1.999$) and a variance of $0$ ($0$). Together with the energy and statistical calculations, the performance analysis showed only a small difference in time consumed by the analytical and numerical approach. The analytical algorithm used $5.3$ ($6.1) seconds, while the numerical approach used $5.8$ ($6.3$) seconds. Comparing the two sampling methods in question, the Metropolis sampling was faster than the Importance sampling method for this particular system. 

As shown in our previous work [@Vmc-bosonic2021], importance sampling generally used more time, and this is once again shown here.

<!-- The effect of blocking on the results Metropolis vs Importance-->

## Variational parameters
<!-- Two particle system-->

The variational parameters were obtained using the steepest gradient decent method with the Jastrow factor. The results for the two electron system are to be found in Table [@tbl:results-variational-parameters-2N], while calculations of $6$ and $12$ electrons are listed in Table [@tbl:results-variational-parameters-larger-sys]. With the Jastrow factor, both $\alpha$ and $\beta$ are to be variated during the same simulation. This has led to immense debugging as the beta value simply will not converge, except when it hits zero. We have gone over the code for days without finding what can be the cause of this, and made the tough decision that we could not spend more time on the issue. $\alpha$ still seems to converge, however because the energy from the monte carlo solver is not accurate enough, the SGD seems to have some issues reaching the lowest energy state.

Before doing the big run, different learning rates for the SGD was performed, and we saw no stability gains when having lower learning rates and more iterations in the SGD. Again, we believe that this ties back to the accuracy of the MC cycling, and the fact that something regarding the $\beta$ variable is malfunctioning.

<!-- Maybe write something about the influence of the step size, how it influences the steepest decent method performance - smaller step size = higher accuracy and more likely to hit the lowest energy, while a higher step size gives a higher performance/uses less time, but is less likely to hit bottom of the energy -->


<!--Higher number of particles: Comment if something was done differently compared to two particle system - e.g., using grid of alphas/betas instead of steepest gradient descent-->


## Minimum energy and particle distance
<!-- Two particle system-->

<!-- Compare values to Taut's article. E.g. for omega = 1, the energy should be 3 au. Then maybe give a deviation \% from Taut’s (2 omega). Also compare with and without the Jastrow factor and with and without interaction (Hamiltonian)-->

The results from the energy minima calculations using the optimal variational parameters are also listed in Table [@tbl:results-variational-parameters-2N] and [@tbl:results-variational-parameters-larger-sys] for 2 and 6 and 12 particles, respectivly.  

<!-- Mean distance between two electrons. Should be dependent on omega(frequency). Is there any dependence on the energy, e.g., higher energy allows for a shorter distance?? -->

The obtained mean distance between two fermions shows to be strongly (inverse) **dependent/independent** on the frequency, $\omega$, which is **expected/unexpected**. Decreasing the frequency with a factor of $FILL$ increases the distance with a factor of $FILL$. Looking at the energy, there is a **decrease/increase** in energy when **decreasing** the frequency. 

<!-- Dependence of omega on kinetic energy - HO has energy steps of $\frac{1}{2}\hbar \omega$, so the kinetic energy should increase with omega-->


<!--Higher number of particles -->

<!-- Dependence of omega on (kinetic) energy - HO has energy steps of $\frac{1}{2}\hbar \omega$, so the kinetic energy should increase with omega-->

<!-- Comment on the effect of using/not using the Jastrow factor  + time consumption -->

<!-- Viral theorem  - compare the analytical results with the viral theorem. Viral theorem does not take into consideration the interaction between the particles--> 

## One Body density
<!-- Two particle system-->

<!-- With and without the Jastrow factor - could also be interesting to compare with and without electron interaction. Is there a dependency of the distance between the particles, the density should probably be higher when the particles are closer together --> 

The results from the one body density calculations utilizing optimal parameters are shown in Figure [@fig:one-body-densities]. With perturbation (interaction term), there is a (small?) **increase/decrease** 

<!--Higher number of particles-->

<!--Comment on the difference between N = 2 vs. 6, 12 (20) + comment on the effect of distance between the particles (with Jastrow and interaction off) -->


## Frequency dependent energy calculations

As seen in table [@tbl:freq-dep-energies-2N] the expecation value of the kinetic and potential energy **increases/decreases** with increasing frequency. <!-- Is it the same for N = 2, 6 and 12? -->.  <!-- How does it behvave compared to the Virial theorem?-->

## Larger systems and bugs in the code

We experienced a lot of trouble with getting the larger systems ($N > 2$) to work. The issue was that the Greens factor evaluated to $0$ with every step, leaving us with no results to do anything with. We suspect the issue is with our evaluation of the Slater gradient, and subsequently with how our quantum force is found, in order to do a step. We were not, however, able to locate this issue in due time, which left us in the awkward situation of not having anything to present.

## Performance analysis
Table [@tbl:results-performence-analysis] gives an overview of the performance analysis of running our algorithm with and without vectorization and parallelization for $N = 6$ electrons. 

<!-- Write something about using different flags-->

<!-- Write something about how we use parallelization (running one experiment on core??) and if/how it gives a speed-up when running our algorithm. Maybe something about how it can be improved in the future.--> 


