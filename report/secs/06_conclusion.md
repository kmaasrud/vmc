# Conclusion

<!-- Summarize what we have done--> 
The well-known Variational Monte Carlo algorithm has been implemented to perform calculations on confined electrons with closed shell configurations in a quantum dot. To find the energy closest to the ground state, two variational parameters were introduced. Using the steepest gradient decent these two are found for different vibrational frequencies. The results are compared with both results from calculations utilizing analytical expressions and values from literature. 

The comparison shows a **significant**(?) speedup using the analytical expression compared to numerical calculations. The energies **are more/less off** for the numerical approach. Hence, if an analytical expression is obtainable, it is recommended implementing this, instead of the using the numerical approach. This applies especially to higher order (dimension and number of particles) systems. 

<!-- Say something about the different performance of Brute force vs. Importance sampling--> 
For $N = 2$ electrons we compared the performance of two sampling methods, brute force Metropolis, and Importance sampling. They show similar results and the former produced them faster. From our previous work [@Vmc-bosonic2021] we know that the performance of importance sampling will rapidly catch up and overtake for a higher number of particles. 

<!-- Effect of the Jastrow factor: particle distance, one-body density, energy --> 
Large parts of this project involve studying the influence the Jastrow factor has on various parameters. Looking at the energy, there is a **$FILL$ higher/lower** energy with the Jastrow factor, compared to without. It gives a **higher/lower** accuracy and the performance **increase/decrease**. The effect of the one-body densities is even **greater(/less) **, as seen in Figure [@fig:one-body-densities]. For the distance between the particles, there is an (a) **increase/decrease** utilizing the Jastrow factor. Hence, if accuracy is important for calculations, the Jastrow factor should be **used/turned off** and <!-- say something about if it the performance is more important - just when to use the factor and when one should not-->

<!-- Comment on what happens using a higher number of particles - time, algorithms, problems, accuracy--> 
As expected, increasing the number of particles gives more time-consuming calculations, both du to the utilization of the slater determinant and the non-existing analytical expressions. (**Check (and prob rewrite this**)
Looking at the one-body density, one sees a **increased/decreased** density with increasing number of particles. This is also **expected/unexpected**. Maybe more importantly, the energy calculation gets a **lower/higher/unaffected** accuracy (compared to Taut's work). 

<!-- Say something about the variational parameters (using two instead of one)--> 
Looking at the two variational parameters, $\alpha$ and $\beta$, we find several sets of the two yielding lower energies. **Check this**

<!-- Effect of the interaction/perturbation: energy, distance?,  one-body density if we have calculated it - this is maybe unnecessary--> 


<!-- Dependency of different frequencies --> 
The frequency dependent calculations shows a great impact on the energy, as expected. Increased frequency gives a **increase/decrease** in energy, where the **potential/kinetic** is affected the most. 

<!-- Optimization --> 
Running a performance analysis for $N = 6$ particles with and without different optimization flags we found that **FILL** worked the best, as seen in Table [@tbl:results-performence-analysis]. 

<!-- Future work/improvements  - important --> 
For future work, we should prioritize debugging our program to make it work properly in all scenarios. This ended up being a difficult task, and one we were not able to fully finish during the writing of this report. The performance of our code can also surely be improved further by properly parellellizing the core solver. In addition, we could've split the determinant calculations into the ones pertaining to spin up and spin down respectively, reducing the number of operations by half. Both of these proved too time-consuming for our deadline.
