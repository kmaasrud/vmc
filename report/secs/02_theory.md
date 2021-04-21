# Theory
Electrons in a confined, two dimensional harmonic oscillator potnetial with the given (idealized) Hamiltionian below is in quantum mechanics called quantum dots. 

$$ \hat{H}  = \sum_{i=1}^{N}{\left( -\frac{1}{2}\nabla^2_i + \frac{1}{2}\omega^2 r_i^2 \right) } + \sum_{i<j}\frac{1}{r_{ij}}$$,

where $r_{ij} = |r_1 - r_2|$, the distance between the electrons. Natural units ($\hbar = c = e = m_e = 1$) are used and energies are in atomic units (a.u). The first term/sum of the hamiltonian is the harmonic oscillator part, well known from  quantum mechanic. It is whats called the unpertubated part. Because electrons repels each other, a repulsion term(the second sum) is added. This term is whats called the pertubation of the system. The modulus of the positions of the electrons ( for a given electron i) as

$$ r_i = \sqrt{r_{i_x}^2 + r_{i_y}^2}$$.

The system will be utilized for closed shells, ie. N = 2, 6, 12 and 20 electrons. 

### Wavefunction
The wavefunction for a two dimentional system with the Harmonic Osccilator potential is given by 

$$\Phi_{n_x, n_y} (x,y) = A H_{n_x} (\sqrt{\omega} x) H_{n_y}(\sqrt{\omega}y \exp{\left[-\frac{\omega}{2}(x^2 + y^2)\right]}$$

where $H_{n_x}$ are Hermite polynomials, and A is the normaliation constant. For the lowest lying state $n_x = n_y = 0$ and hence the energy is $\epsilon_{n_x, n_y} = \omega(n_x + n_y + 1 = \omega$.  

The energy of the ground state for two electrons without interaction, is simply the sum of the energies:  $\epsilon_{n_x,n_y} = 2\times (0 + 0 + 1) =  2\omega$. 

The wavefunction for the unpartubated stystem is given by
$$
\Phi (\mathbf{r_1}, \mathbf{r_2} ) = C \exp{[-\frac{\omega}{2}(\mathbf{r_1}^2 + \mathbf{r_2}^2)]}
$$
where $\mathbf{r_i} = \sqrt{r{i_x}^2 + r_{i_y}^2}$.  The total spin in the ground state is simply zero as the two electrons living in the state is pared with opposite spins (eg. $\pm 1/2$). 

The ground state energy is given by the unpartubated system. Adding a pertubation/interaction will rise the energy. For the simplest system with two electrons, this pertubation can be found through partubation theory, whilst for a higher number of particles, other measurments/actions must be taken to find the energy(??)

