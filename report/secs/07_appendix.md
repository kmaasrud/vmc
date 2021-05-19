\clearpage
\appendix

# Definitions and notation

## Hermite polynomials {#sec:hermite}

The Hermite polynomials are the solutions to the following contour integral [@ArfkenWeber2005]:

$$ H_n(z) = \frac{n!}{2\pi i}\oint e^{-r^2 - tz}t^{-n-1}dt. $$

In this report, we will consider the real Hermite polynomials, and only up to a given order. A computationally efficient way of finding these is given by the following sequence:

$$ H_n(x) = c_{n + m}x^n + c_{n-1 + m}x^{n-1}... + c_{m}x^0, \qquad m = \sum_{i=1}^n i$$

where the coefficients $c_n$ are given by the triangle sequence shown in ([@eq:jovovic]).

$$c_i = 1, 0, 2, -2, 0, 4, 0, -12, 0, 8, 12, 0, -48, ...$$ {#eq:jovovic}

This is just a a selection of the sequence, more can be found from the work by @Jovovic2001.

## Index notation for sums and products {#sec:index-notation}

For products and sums, the following convention is used:

$$\sum_{i <j}^N = \sum_{i=1}^N \sum_{j=i+1}^N,\quad \text{or}\quad \prod_{i <j}^N = \prod_{i=1}^N \prod_{j=i+1}^N$$

# Derivations

## Local energy of two-fermion system {#sec:local-energy-derivation}

*Do the stuff*, as Amund would say <3


## Analytical Laplace of the Trial wavefunction

The trial wavefunction of a two-particle system is

$$ \Psi_T (\mathbf{r_1}, \mathbf{r_2}) = \Psi_1  * \Psi_2 = C \exp{(-\alpha \omega (r_1^2 + r_2^2)/2)} \exp{\left( \frac{ar_{12}}{1 + \beta r_{12}}\right)} $$

as defined in theory section [@sec:theory-local-energy].

The Laplacian is the double derivative in all dimensions, defined as:

$$
\Delta f=\frac{\partial^{2} f}{\partial x^{2}}+\frac{\partial^{2} f}{\partial y^{2}}+\frac{\partial^{2} f}{\partial z^{2}}
$$

The calculations:
First we change the laplacian to work with a cartesian 2D system:

$$
\Delta f=\frac{\partial^{2} f}{\partial x_1^{2}}+\frac{\partial^{2} f}{\partial x_2^{2}}+\frac{\partial^{2} f}{\partial y_1^{2}}+\frac{\partial^{2} f}{\partial y_2{2}}
$$

$$
C\mathrm{e}^{\frac{s\sqrt{\left(C-x_1\right)^2+\left(u-y_1\right)^2}}{b\left|x_1-C\right|+1}-\frac{aw\left(x_1^2+y_1^2+u^2+C^2\right)}{2}}\left(-\dfrac{s\left(C-x_1\right)}{\sqrt{\left(C-x_1\right)^2+\left(u-y_1\right)^2}\left(b\left|x_1-C\right|+1\right)}-\dfrac{bs\sqrt{\left(C-x_1\right)^2+\left(u-y_1\right)^2}\left(x_1-C\right)}{\left|x_1-C\right|\left(b\left|x_1-C\right|+1\right)^2}-awx_1\right)^2+C\mathrm{e}^{\frac{s\sqrt{\left(C-x_1\right)^2+\left(u-y_1\right)^2}}{b\left|x_1-C\right|+1}-\frac{aw\left(x_1^2+y_1^2+u^2+C^2\right)}{2}}\left(-\dfrac{s\left(C-x_1\right)^2}{\left(\left(C-x_1\right)^2+\left(u-y_1\right)^2\right)^\frac{3}{2}\left(b\left|x_1-C\right|+1\right)}+\dfrac{s}{\sqrt{\left(C-x_1\right)^2+\left(u-y_1\right)^2}\left(b\left|x_1-C\right|+1\right)}+\dfrac{2bs\left(C-x_1\right)\left(x_1-C\right)}{\sqrt{\left(C-x_1\right)^2+\left(u-y_1\right)^2}\left|x_1-C\right|\left(b\left|x_1-C\right|+1\right)^2}+\dfrac{2b^2s\sqrt{\left(C-x_1\right)^2+\left(u-y_1\right)^2}}{\left(b\left|x_1-C\right|+1\right)^3}-aw\right)
$$

