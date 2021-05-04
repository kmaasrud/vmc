# Theory

Electrons in a confined, two dimensional harmonic oscillator potnetial with the given (idealized) Hamiltionian below is in quantum mechanics called quantum dots. 

$$ H = \sum_{i=1}^{N}{\left( -\frac{1}{2}\nabla^2_i + \frac{1}{2}\omega^2 |\mathbf r_i|^2 \right) } + \sum_{i<j}\frac{1}{r_{ij}}$${#eq:hamiltonian}

where $r_{ij} = |r_i - r_j|$ is the distance between two electrons. We use the natural units $\hbar = c = e = m_e = 1$ and all energies are in atomic units (a.u).

The first term of the hamiltonian is a simple harmonic oscillator potential. Because electrons repel each other, we also get a repulsive term as part of the Hamiltonian - the so-called perturbation of the system. 

## Wavefunction

The two-dimensional solution for a single particle in a harmonic oscillator potential (consider equation ([@eq:hamiltonian]) for a single particle) is the following wave function:

$$\phi_{n_x, n_y} (x,y) = A H_{n_x} (\sqrt{\omega} x) H_{n_y}(\sqrt{\omega}y) \exp{\left[-\frac{\omega}{2}(x^2 + y^2)\right]}.$$

where $H_{i}$ are Hermite polynomials (see [@sec:hermite]), and $A$ is the normalization constant. For the lowest lying state, we have $n_x = n_y = 0$ and hence the energy $\epsilon_{n_x, n_y} = \omega(n_x + n_y + 1) = \omega$. 

The total wave function for a non-interacting two-electron system is therefore given as:

$$ \Phi (\mathbf r_1, \mathbf r_2 ) = C \exp \left[- \frac{\omega}{2}\left(|\mathbf r_1|^2 + |\mathbf r_2|^2\right)\right], $$

with an energy of $2\omega$. The total spin in the ground state is simply zero as the two electrons living in the state is pared with opposite spins (eg. $\pm 1/2$). <!-- Is this correct?. -->

The ground state energy is given by the unperturbed system. Adding a pertubation/interaction will rise the energy. For the simplest system with two electrons, this pertubation can be found through perturbation theory, whilst for a higher number of particles, other measurments or actions must be taken to find the energy(??).

## Local energy

By definition, the local energy is given by 

$$ E_l = \frac{1}{\Psi_T} \hat{H} \Psi_T $$

$\Psi_T$ is the trial wavefunction of the system. The Hamiltionian is given by equation ([@eq:hamiltonian]) and the trial wavefunction is 

$$ \Psi_T (\mathbf{r_1}, \mathbf{r_2}) = \Psi_1  * \Psi_2 = C \exp{(-\alpha \omega (r_1^2 + r_2^2)/2)} \exp{\left( \frac{ar_{12}}{1 + \beta r_{12}}\right)} $$

Where $a = 1$ when the two electrons in question have anti- parallell spins and $a = 1/3$ when the spins are parallell. $\alpha, \beta$ are the variational parameters. 

Hence, the local energy is shown to be ( see appendix) 

$$\begin{aligned}
E_L &= 2 \alpha \omega + \frac{1}{2} + \omega^2 (1 - \alpha^2) (r_1^2 + r_2^2) \\
&- \frac{a}{(1 + \beta r_{12})^2} \left( -\alpha \omega r_{12} + \frac{a}{(1 + \beta r_{12})^2} + \frac{1 - \beta r_{12}}{r_{12}(1 + \beta r_{12})}\right) + \frac{1}{r_{12}}.
\end{aligned}$$ {#eq:analytic-local-energy}

Equation ([@eq:analytic-local-energy]) is our analytic expression for the local energy of the two electron system. 

The numerical local (kinetic) energy is calculated using the derivitive of the velocity utilizing the two point approximation of the first derivative

$$ \frac{d g(x)}{dx} \approx \frac{g(x + \Delta x) - g ( x - \Delta x)}{2 \Delta x}  $$

Second derivative by three point approximation

$$ \frac{d g(x)}{dx} \approx \frac{g(x + \Delta x) - 2 g(x)  + g ( x - \Delta x) }{ \Delta x^2}  $$

$\Delta x$ is the stepsize which we let run towards zero. The error is proportional to $(\Delta x ^2 )$.


## Testing

Testing in Rust is normally divided in two categories: *unit tests* and *integration tests*. Unit tests are small codes to test specific functions inside the code. These tests are normally written in the same file as the functions themselves, but inside a module annotated with ```cfg(test)```.

On the other hand, integration tests are written externally to the library, and is made to test the integration of the functions in the program. These tests are often much larger than unit tests, and are made to make sure that the internal functions works well together from the standpoint of an external user. Therefore, integration tests are normally written in a separate ```tests``` directory at the same level as the ```src``` directory.

More on testing can be found in the official documentation of the Rust programming language[@Rust-docs-testing].

