# Theory

We consider a system of electrons situated in an isotropic harmonic oscillator potential. We will use Hartree's atomic units[^hartree] to get the idealized Hamiltonian presented below:

$$ H = \sum_{i=1}^{N}{\left( -\frac{1}{2}\nabla^2_i + \frac{1}{2}\omega^2 |\mathbf r_i|^2 \right) } + \sum_{i<j}\frac{1}{r_{ij}}. $$ {#eq:hamiltonian}

Here $r_{ij} = |r_i - r_j|$ is the distance between two electrons. The first sum is the single particle harmonic oscillator potential. Because electrons repel each other, we also get the latter repulsive sum as part of the Hamiltonian - the perturbation of the system.

[^hartree]: $\hbar = c = e = m_e = 1$, see [@Hartree1928].

## The unperturbed wave function {#sec:unperturbed}

Disregarding interactions, there is a closed-form solution for the Hamiltonian shown in equation ([@eq:hamiltonian]) for a single particle. The solutions follows [@Project2]:

$$\phi_{n_x, n_y} (x, y, \alpha) = A H_{n_x} (\sqrt{\alpha\omega} x) H_{n_y}(\sqrt{\alpha\omega}y) \exp{\left[-\frac{\alpha\omega}{2}(x^2 + y^2)\right]}.$${#eq:spwf}

Here, $H_{n_i}$ are Hermite polynomials (see [@sec:hermite]), and $A$ is the normalization constant. For the lowest lying state, we have $n_x = n_y = 0$ and hence the energy of a non-interacting fermion $\epsilon$ is:

$$ \epsilon_{n_x, n_y} = \omega(n_x + n_y + 1) = \omega. $$ {#eq:non-interacting-energy}

The Pauli exclusion principle states that two fermions can not occupy the same quantum state simultaneously. For each state $(n_x, n_y)$ a fermion may have spin up or down, which means it can be occupied by at most two fermions. Using this principle, the ground state energies of the closed-shell configurations $N = 2, 6, 12$ and $20$ can easily be calculated using equation ([@eq:non-interacting-energy]). The energies are given in table [@tbl:energies].


| **Number of particles $N$** | **$E$ [a.u]**        |
| ----:                       | ---                  |
| $2$                         | $2 \omega$           |
| $6$                         | $10 \omega$          |
| $12$                        | $28 \omega$          |
| $20$                        | $60 \omega$          |

Table: The ground state energy of $N$ non-interacting particles in an isotropic harmonic potential well. $\omega$ is the oscillator frequency. Energies are given in Hartree's atomic units.  {#tbl:energies}

These energies serve as great values to benchmark our program against.

## The complete wave function
<!-- Should we really call it the "complete" wave function? I don't know, it sounds right to me atm, but perhaps it's a bit redundant... -->

Single harmonic oscillators are solvable analytically, but introducing the repulsive perturbation forces us to tackle the problem differently. We choose a variational Monte Carlo approach, and use the Slater-Jastrow type of trial wave function, namely

$$ \Psi_T(\mathbf R, \alpha, \beta) = \Psi_D \Psi_J = \det(D(\mathbf R, \alpha))\exp(J(\mathbf R, \beta)), $$ 

where $D(\mathbf R)$ is a Slater matrix and $J(\mathbf R)$ is a Padé-Jastrow correlation function. $\mathbf R$ here represents the set of all the individual particle's positions, and $\alpha$ and $\beta$ are the variational parameters. Following @Project2, our ansatz for the factors of this trial wave function is:

$$\begin{aligned}
\Psi_D &= \det(D(\mathbf R, \alpha)),\qquad D_{ij} = \phi_j(\mathbf r_i, \alpha), \\
\Psi_J &= \prod_{i<j}^N \exp\left(\frac{ar_{ij}}{1 + \beta r_{ij}}\right).
\end{aligned}$$ {#eq:trial-ansatz}

$\phi_j(\mathbf r_i)$ is the single particle wave function for the $i$-th fermion, as shown in ([@eq:spwf]), with $j$ being an index describing each unique quantum state[^quantum]. The coefficient $a = 1$ when the electrons $i$ and $j$ have anti-parallel spins, and $a = \frac{1}{3}$ when their spins are parallel. The index notation on the product is as explained in [@sec:index-notation].

[^quantum]: E.g. $(0,0,\uparrow)$, $(2,1,\downarrow)$, etc.

### A system of $N=2$ fermions

Expanding the ansatz ([@eq:trial-ansatz]) for a system of two fermions, the trial wave function is reduced to:

$$ \Psi_T (\mathbf r_1, \mathbf r_2 ) = C \exp \left(- \frac{\alpha\omega \left(|\mathbf r_1|^2 + |\mathbf r_2|^2\right)}{2}\right) \exp\left(\frac{ar_{12}}{1 + \beta r_{12}}\right). $$ {#eq:analytical}

The total spin in the ground state of this system is simply zero as the two fermions are paired with opposite spins.

## Local energy {#sec:theory-local-energy}

We define the *local energy* of our wave function as:

$$ E_L \equiv \frac{1}{\Psi}H\Psi. $$

As shown in [@sec:two-fermion-derivation], the local energy for a two-fermion system is:

$$ \begin{aligned}
E_L &= 2 \alpha \omega + \frac{1}{2} + \omega^2 (1 - \alpha^2) (r_1^2 + r_2^2) \\
&- \frac{a}{(1 + \beta r_{12})^2} \left( -\alpha \omega r_{12} + \frac{a}{(1 + \beta r_{12})^2} + \frac{1 - \beta r_{12}}{r_{12}(1 + \beta r_{12})}\right) + \frac{1}{r_{12}}.
\end{aligned} $$

## Quantum Force 

Importance sampling requires the quantum force, which for the two-fermion case is given by

$$ F = -2 \alpha \omega \mathrm{r}_{1}+\frac{2 a}{r_{12}\left(1+\beta r_{12}\right)^{2}} \mathrm{r}_{12}-2 \alpha \omega \mathrm{r}_{2}+\frac{2 a}{r_{12}\left(1+\beta r_{12}\right)^{2}} \mathrm{r}_{21}, $$

or generally as

$$F = 2\frac{\nabla \Psi_T}{\Psi_T}$$

as shown in Appendix [@sec:two-fermion-derivation].

## One-body density
<!-- Write some more here? Find a reference at least - Anna -->
The one-body density can aid in visualizing and optimizing the wavefunction, as well as calculating exitation energies. It describes the probability of finding any of the $N$ electrons in the volume $d \mathbf{r}_1$. The density is defined as [@Hogberget2013]:

$$\rho(\mathbf{r_1}) = \int_{\mathbf{r}_2} \int_{\mathbf{r}_3} \cdots \int_{\mathbf{r}_N}|\Psi(\mathbf{r_1, ... , r}_N)|^2 d\mathbf{r_2}...d\mathbf{r}_N. $$
{#eq:one-body-density}

Notice that we integrate $|\psi|^2$ over all particles but the one we are considering. The one-body density is normalized over the number of particles.

By the Pauli principle, no electrons can occupy the same state, making the integral a bit simpler. This is accounted for by the repulsion/pertubation term in the Hamiltonian. 

## The Virial Theorem

<!-- For comparison later, see exercise f-->
The Virial theorem relates the avarage time $\langle T \rangle$ for the total kinetic and potential energy in a system of $N$ particles (or planets) by the following equation

$$ \langle T \rangle  = - \frac{1}{2} \sum_{k=1}^N \langle \mathbf{F_k \cdot r_k} \rangle, $$

where $\mathbf{F}_k$ and $\mathbf{r}_k$ is the is the force on and position of particle $k$ respectively. The theorem allows for calculations of the average total kinetic energy of complex systems, independent of the temperature. 

A more relevant quentum mechanical version of the theorem arises when we consider the potential around the confined particles instead of the force. Hence,

$$ 2 \langle T \rangle = - \frac{1}{2} \sum_n \langle X_n \frac{dV}{dX_n} \rangle.$$

For a simple Harmonic oscillator, the potential, $V$, is

$$V = \frac{1}{2} k x^2$$. 

<!-- source: https://en.wikipedia.org/wiki/Virial_theorem -->
