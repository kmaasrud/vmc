# Theory

We consider a system of electrons situated in an isotropic harmonic oscillator potential. We will use Hartree's atomic units[^hartree] in order to get the idealized Hamiltonian presented below:

$$ H = \sum_{i=1}^{N}{\left( -\frac{1}{2}\nabla^2_i + \frac{1}{2}\omega^2 |\mathbf r_i|^2 \right) } + \sum_{i<j}\frac{1}{r_{ij}}. $$ {#eq:hamiltonian}

Here $r_{ij} = |r_i - r_j|$ is the distance between two electrons and $r_{i} = \sqrt{r_{i_x}^2 + r_{i_y}^2}$ is electron i's position. The first sum is the single particle harmonic oscillator potential. Because electrons repel each other, we also get the latter repulsive sum as part of the Hamiltonian - the perturbation of the system.

[^hartree]: $\hbar = c = e = m_e = 1$, see [@Hartree1928].

## The unperturbed wave function {#sec:unperturbed}

Disregarding interactions, there is a closed-form solution for the Hamiltonian shown in equation ([@eq:hamiltonian]) for a single particle. The solutions follows [@Project2]:

$$\phi_{n_x, n_y} (x,y) = A H_{n_x} (\sqrt{\omega} x) H_{n_y}(\sqrt{\omega}y) \exp{\left[-\frac{\omega}{2}(x^2 + y^2)\right]}.$$

Here, $H_{n_i}$ are Hermite polynomials (see [@sec:hermite]), and $A$ is the normalization constant. For the lowest lying state, we have $n_x = n_y = 0$ and hence the energy of a non-interacting fermion $\epsilon$ is:

$$ \epsilon_{n_x, n_y} = \omega(n_x + n_y + 1) = \omega. $$ {#eq:non-interacting-energy}

The Pauli exclusion principle states that two fermions can not occupy the same quantum state simultaneously. For each state $(n_x, n_y)$ a fermion may have spin up or down, which means it can be occupied by at most two fermions. Using this principle, the ground state energies of the closed-shell configurations $N = 2, 6, 12$ and $20$ can easily be calculated using equation ([@eq:non-interacting-energy]). The energies are given in table [@tbl:energies].

<!-- Don't worry about the erroneous table numbering. This is a fault of a Doctor filter I made, but a quick-fix is on the to do list <3. -->

| **Number of particles $N$** | **$E$ (a.u)** |
| ----:                       | ---                  |
| $2$                         | $2 \omega$           |
| $6$                         | $10 \omega$          |
| $12$                        | $28 \omega$          |
| $20$                        | $60 \omega$          | 

Table: The ground state energy of $N$ non-interacting particles in an isotropic harmonic potential well. $\omega$ is the oscillator frequency. Energies are given in Hartree's atomic units. {#tbl:energies}

These energies serve as great values to benchmark our program against.

## The complete wave function
<!-- Should we really call it the "complete" wave function? I don't know, it sounds right to me atm, but perhaps it's a bit redundant... -->

Single harmonic oscillators are solvable analytically, but introducing the repulsive perturbation forces us to tackle the problem differently. We choose a variational Monte Carlo approach, and use the Slater-Jastrow type of trial wave function, namely

<!-- TODO: Not yet completely sure whether exp(J(R)) is correct. -->
$$ \Psi_T(\mathbf R, \alpha, \beta) = \Psi_D \Psi_J = \det(D(\mathbf R, \alpha))\exp(J(\mathbf R, \beta)), $$

where $D(\mathbf R)$ is a Slater matrix and $J(\mathbf R)$ is a Padé-Jastrow correlation function. $\mathbf R$ here represents the set of all the individual particle's positions, and $\alpha$ and $\beta$ are the variational parameters. Following @Project2, our ansatz for the factors of this trial wave function is:

$$\begin{aligned}
\Psi_D &= \det(D(\mathbf R)),\qquad D_{ij} = \phi_j(\mathbf r_i), \\
\Psi_J &= \prod_{i<j}^N \exp\left(\frac{ar_{ij}}{1 + \beta r_{ij}}\right).
\end{aligned}$$ {#eq:trial-ansatz}

$\phi_j(\mathbf r_i)$ is the single particle wave function for the $i$-th fermion, as described in [@sec:unperturbed], with $j$ being an index describing each unique quantum state[^quantum]. The coefficient $a = 1$ when the electrons $i$ and $j$ have anti-parallel spins, and $a = \frac{1}{3}$ when their spins are parallel. The index notation on the product is as explained in [@sec:index-notation].

[^quantum]: E.g. $(0,0,\uparrow)$, $(2,1,\downarrow)$, etc.

### A system of $N=2$ fermions

Expanding the ansatz ([@eq:trial-ansatz]) for a system of two fermions, the trial wave function is reduced to:

$$ \Psi_T (\mathbf r_1, \mathbf r_2 ) = C \exp \left(- \frac{\alpha\omega \left(|\mathbf r_1|^2 + |\mathbf r_2|^2\right)}{2}\right) \exp\left(\frac{ar_{12}}{1 + \beta r_{12}}\right). $$

The total spin in the ground state of this system is simply zero as the two fermions are paired with opposite spins.

#### Local energy {#sec:theory-local-energy}

We define the *local energy* of a wave function as:

$$ E_L \equiv \frac{1}{\Psi}H\Psi. $$

As shown in [@sec:local-energy-derivation], the local energy for a two-fermion system is:

$$ \begin{aligned}
E_L &= 2 \alpha \omega + \frac{1}{2} + \omega^2 (1 - \alpha^2) (r_1^2 + r_2^2) \\
&- \frac{a}{(1 + \beta r_{12})^2} \left( -\alpha \omega r_{12} + \frac{a}{(1 + \beta r_{12})^2} + \frac{1 - \beta r_{12}}{r_{12}(1 + \beta r_{12})}\right) + \frac{1}{r_{12}}.
\end{aligned} $$ <!-- Is this correct?. -->

The numerical local (kinetic) energy is calculated using the derivitive of the velocity utilizing the two point approximation of the first derivative

$$ \frac{d g(x)}{dx} \approx \frac{g(x + \Delta x) - g ( x - \Delta x)}{2 \Delta x} $$

Second derivative by three point approximation

$$ \frac{d g(x)}{dx} \approx \frac{g(x + \Delta x) - 2 g(x)  + g ( x - \Delta x) }{ \Delta x^2}  $$

$\Delta x$ is the stepsize which we let run towards zero. The error is proportional to $(\Delta x ^2 )$.

#### Quantum Force 

Importance sampling requires the quantum force, which for the two electron case is given by(derived in Appendix [@sec:two-fermion-derivation])

$$
F = -2 \alpha \omega \mathrm{r}_{1}+\frac{2 a}{r_{12}\left(1+\beta r_{12}\right)^{2}} \mathrm{r}_{12}-2 \alpha \omega \mathrm{r}_{2}+\frac{2 a}{r_{12}\left(1+\beta r_{12}\right)^{2}} \mathrm{r}_{21}
$$

## Slater determinant

The slater determinant is a crucial, time consuming part of the trail wavefunction and hence the metropolis algorithm, in evaluating the quantum force, and when computing the local energy and other observebales.  Standard Gaussian elimination determinant calculation for a $N \times N$ matrix is in the ordrer of $N^3$.  Our gradient and Laplacien requiers $N \cdot dim$ determinant calculations. Hence, it is important to optimize. 

Calcutating the trasition probability of the trial wavefunction $\Psi_{old}(\mathbf{R}) / \Psi_{new}(\mathbf{R})$ requieres a computation of the ratio of the determinants $det(D_{old}(\mathbf{R})) / det(D_{new}(\mathbf{R}))$. Insted of recalculate the whole determinant for each step, the algorithm can be optimized using Sherman-Morrison formula, reducing the computational cost of evaluating the ratio of the determinants with a factor of $N$ of the move is accepted.  


