\clearpage
\appendix

# Appendix

## Hermite polynomials {#sec:hermite}

The Hermite polynomials are the solutions to the following contour integral [@ArfkenWeber2005]:

$$ H_n(z) = \frac{n!}{2\pi i}\oint e^{-r^2 - tz}t^{-n-1}dt. $$

In this report, we will consider the real Hermite polynomials, and only up to a given order. A computationally efficient way of finding these is given by the following sequence:

$$ H_n(x) = c_{n + m}x^n + c_{n-1 + m}x^{n-1}... + c_{m}x^0, \qquad m = \sum_{i=1}^n i$$

where the coefficients $c_n$ are given by the triangle sequence shown in ([@eq:jovovic]).

$$c_i = 1, 0, 2, -2, 0, 4, 0, -12, 0, 8, 12, 0, -48, ...$$ {#eq:jovovic}

This is just a a selection of the sequence, the rest is fetched from the work by @Jovovic2001.

## Index notation for sums and products {#sec:index-notation}

For products and sums, the following convention is used:

$$\sum_{i <j}^N = \sum_{i=1}^N \sum_{j=i+1}^N,\quad \text{or}\quad \prod_{i <j}^N = \prod_{i=1}^N \prod_{j=i+1}^N$$

## Analytical Laplace of the Trial wavefunction

The trial wavefunction of a two-particle system is

$$ \Psi_T (\mathbf{r_1}, \mathbf{r_2}) = \Psi_1  * \Psi_2 = C \exp{(-\alpha \omega (r_1^2 + r_2^2)/2)} \exp{\left( \frac{ar_{12}}{1 + \beta r_{12}}\right)} $$

as defined in theory section [@sec:theory-local-energy].

The Laplacian is the double derivative in all dimensions, defined as:

$$
\Delta f=\frac{\partial^{2} f}{\partial x^{2}}+\frac{\partial^{2} f}{\partial y^{2}}+\frac{\partial^{2} f}{\partial z^{2}}
$$

The calculations:

$$
Calculate all the shit
$$

boop
