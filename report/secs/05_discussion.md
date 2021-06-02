# Discussion

## Verification/ Two electrons

As a vailidation test for our algorithm we calculated the energy without the Jastrow factor for $N = 2$ electrons, expecting a energy of $2.0 $ a.u and a variance of zero. Running this system with the Metropolis (and Importance sampling) method returned a energy of $FILL$ ($FILL$) and a variance of $FILL$ ($FILL$). Togheter with the energy and statistical calculations, a performance analysis showed a **FILL- large, intermediate..** difference in time consumtion. The analytical algorithm used $FILL$ seconds, wile the numerical approach used $FILL$ seconds. Comparing the two sampling methods in question, the **Metropolis/Importance** sampling was **faster/slower** than the **Metropolis/Importance**. 

<!-- Reference to our last project where we preformed performance analysis for different number of particles. Can maybe write something like: This is a two particle system, where we found in our last project Importance/Metropoliis is faster, while for larger system Importance/Metropolis will dominate in performance -->


<!-- The effect of blocking on the results Metropolis vs Importance-->

## Variatonal parameters


The variational parameters were obtained using the steepest decent method both with and without the Jastrow factor. The results are to be found in Table [@tbl:results-variational-parameters].  Without the factor our calculations returned $\alpha = FILL$, with the expectation of $\alpha = 1$. With the Jastrow factor, there is an additional variational parameter, $\beta$. 

<!-- Maybe write something about the influence of the stepsize, how it influences the steepest decent method performance - smaller stepsize = higher accuracy and more likely to hit the lowest energy, while a higher stepsize gives a higher performamce/uses less time, but is less likely to hit bottom of the energy -->

## Energy minima

<!-- Compare values to Taut's article. E.g. for omega  = 1, the energy should be 3 a.u.. Then maybe give a deviation \%  from tauts (2 omega). Also compare with and without the Jastrow factor-->

The results from the energy minima calculatuions are also listed in Table [@tbl:results-variational-parameters]. 


<!-- Mean distance between two electrons. Should be dependent on omega(frequency). Is there any dependence on the energy, eg. higher energy allows for a shorter distance?? -->
The obtained mean distance between two ferimons shows to be strongly (inverse) **dependent/indupendent** on the frequency, $\omega$, which is **expected/unexpected**. Decreasing the frequency with a factor of $FILL$ increases the distance with a factor of $FILL$. Looking at the energy, there is a **decrease/increase** in energy when **decreasing** the freqeuncy. 

## One Body density
<!-- With and without the Jastrow factor - could also be interesting to compare with and without electron interaction. Is there a dependency of the distanve between the particles, the density should probably be higher when the particles are closer togheter --> 

The results from the one body density calculations are shown in Figure [@fig:one-body-densities]. With perturbation (interaction term), there is a (small?) **increas/decrease** 

<!-- Dependence of omega on kinetic energy - HO has energy steps of $\frac{1}{2}\hbar \omega$, so the kinetic energy should increase with omega-->


<!-- Viral theorem  - compare the analytical results with the viral theorem--> 


