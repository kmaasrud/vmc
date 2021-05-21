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

## Analytical derivation of the Quantum Force, Laplacian and Local energy of two-fermion systems {#sec:two-fermion-derivation}

The trial wavefunction of a two-particle system is

$$ \Psi_T (\mathbf{r_1}, \mathbf{r_2}) = \Psi_1  * \Psi_2 = C \exp{(-\alpha \omega (r_1^2 + r_2^2)/2)} \exp{\left( \frac{ar_{12}}{1 + \beta r_{12}}\right)} $$

as defined in theory section [@sec:theory-local-energy].

The Laplacian is the double derivative in all dimensions, defined as:

$$
\Delta f=\frac{\partial^{2} f}{\partial x^{2}}+\frac{\partial^{2} f}{\partial y^{2}}+\frac{\partial^{2} f}{\partial z^{2}}
$$

The calculations:
First we change the laplacian to work with a cartesian twodimensional system:

$$
\Delta f=\frac{\partial^{2} f}{\partial x_1^{2}}+\frac{\partial^{2} f}{\partial x_2^{2}}+\frac{\partial^{2} f}{\partial y_1^{2}}+\frac{\partial^{2} f}{\partial y_2^{2} }
$$

Then the wavefunction is inserted.

$$
\Delta \Psi_T =\frac{\partial^{2} \Psi_T }{\partial x_1^{2}}+\frac{\partial^{2} \Psi_T }{\partial x_2^{2}}+\frac{\partial^{2} \Psi_T }{\partial y_1^{2}}+\frac{\partial^{2} \Psi_T }{\partial y_2^{2}}
$$

We see that the trial wavefunction is composed of two exponential terms, and to do the derivative, we can use the derivative product rule twice.


$$(f  g)^{\prime \prime}= (f^{\prime}  g+f  g^{\prime})^\prime = f^{\prime \prime} g + 2 f^\prime g^\prime + f g^{\prime \prime}$$ {#eq:productruletwice}

where 


$$ 
\begin{aligned}
f =& C \exp{(-\alpha \omega (r_1^2 + r_2^2)/2)}\\
g =& \exp{\left( \frac{ar_{12}}{1 + \beta r_{12}}\right)}
\end{aligned}
$$

The two following equalities are then used to find the first derivative of $f$ and $g$

$$
\frac{\partial r_{1}}{\partial x_{1}}=x_{1} / r_{1},  \quad \frac{\partial r_{12}}{\partial x_{1}}=\left(x_{1}-x_{2}\right) / r_{1}
$$


$$\frac{\partial f}{\partial x_{1}}=-\alpha \omega x_{1} f, \quad \nabla_{i} f=-\alpha \omega f \textbf{r}_{i}$${#eq:fd}

Where $i$ denotes the specific particle, and the particle position $r_i$ equals $(x_i, y_i)$. For the second term $g$ we have

$$\frac{\partial g}{\partial x_{1}}=g \frac{a\left(x_{1}-x_{2}\right)}{r_{12}\left(1+\beta r_{12}\right)^{2}}, \quad \nabla_{i} g=g \frac{a}{r_{12}\left(1+\beta r_{12}\right)^{2}} \textbf{r}_{ij}$${#eq:gd}

Where $j$ is the opposite particle of $i$ and the distance from $j$ to $i$, $\textbf{r}_{ij} = (x_i-x_j, y_i - y_j)$.


From this we can actually find an analytical solution to the *quantum force* used in importance sampling, defined as
$$
F = 2\frac{\nabla \Psi_T}{\Psi_T} = 2 \frac{f^\prime g + f g^\prime}{fg}
$$

$$
F = -2 \alpha \omega \textbf{r}_{1}+\frac{2 a}{r_{12}\left(1+\beta r_{12}\right)^{2}} \textbf{r}_{12}-2 \alpha \omega \textbf{r}_{2}+\frac{2 a}{r_{12}\left(1+\beta r_{12}\right)^{2}} \textbf{r}_{21}
$$

Next, we calculate the Laplacian, or double derivative, of the first term, $f$.

$$\frac{\partial^{2} f}{\partial x_{1}^{2}}=f\left(\alpha^{2} \omega^{2} x_{1}^{2}-\alpha \omega\right), \quad \nabla^{2} f=f\left(\alpha^{2} \omega^{2}\left(r_{1}^{2}+r_{2}^{2}\right)-4 \alpha \omega\right)$${#eq:fdd}

And the second term, $g$.

$$
\begin{aligned}
\frac{\partial^{2} g}{\partial x_{1}^{2}}=g[& \frac{a^{2}\left(x_{1}-x_{2}\right)^{2}}{r_{12}^{2}\left(1+\beta r_{12}\right)^{4}}+\frac{a r_{12}\left(1+\beta r_{12}\right)^{2}}{r_{12}^{2}\left(1+\beta r_{12}\right)^{4}} \\
&\left.-\frac{a\left(x_{1}-x_{2}\right)\left[\left(x_{1}-x_{2}\right) / r_{12}\left(1+\beta r_{12}\right)^{2}+2 r_{12}\left(1+\beta r_{12}\right) \beta\left(x_{1}-x_{2}\right) / r_{12}\right]}{r_{12}^{2}\left(1+\beta r_{12}\right)^{4}}\right]
\end{aligned}
$$

$$
\frac{\partial^{2} g}{\partial x_{1}^{2}}=g\left[\frac{a^{2}\left(x_{1}-x_{2}\right)^{2}}{r_{12}^{2}\left(1+\beta r_{12}\right)^{4}}+\frac{a}{r_{12}\left(1+\beta r_{12}\right)^{2}}-\frac{a\left(x_{1}-x_{2}\right)^{2}}{r_{12}^{3}\left(1+\beta r_{12}\right)^{2}}-\frac{2 a \beta\left(x_{1}-x_{2}\right)^{2}}{r_{12}^{2}\left(1+\beta r_{12}\right)^{3}}\right]
$$

With this, we get


$$
\nabla^{2} g=g\left[\frac{2 a^{2}}{\left(1+\beta r_{12}\right)^{4}}+\frac{4 a}{r_{12}\left(1+\beta r_{12}\right)^{2}}-\frac{2 a}{r_{12}\left(1+\beta r_{12}\right)^{2}}-\frac{2 a \beta}{\left(1+\beta r_{12}\right)^{3}}\right]
$$

Which can be further shortened by pulling $\frac{2a}{(1+\beta r_{12})^2}$ outside the brackets to:

$$\nabla^2g =g \frac{2 a}{\left(1+\beta r_{12}\right)^{2}}\left[\frac{a}{\left(1+\beta r_{12}\right)^{2}}+\frac{1}{r_{12}}-\frac{2 \beta}{1+\beta r_{12}}\right]$${#eq:gdd}

Now, by inserting $f^{\prime \prime}$, $g^{\prime \prime}$, $f^\prime$ and $g^\prime$ from equations @eq:fdd, @eq:gdd, @eq:fd and @eq:gd into equation @eq:productruletwice, we actually obtain the *Laplacian* of the trial wavefunction $\nabla^2 \Psi_T$. First we simplify the middle term:

$$
\begin{aligned}
\nabla f \nabla g &=-f g \frac{a \alpha \omega}{r_{12}\left(1+\beta r_{12}\right)^{2}}\left[x_{1}\left(x_{1}-x_{2}\right)+y_{1}\left(y_{1}-y_{2}\right)-x_{2}\left(x_{1}-x_{2}\right)-y_{2}\left(y_{1}-y_{2}\right)\right] \\
&=-f g \frac{a \alpha \omega}{r_{12}\left(1+\beta r_{12}\right)^{2}}\left[\left(x_{1}-x_{2}\right)\left(x_{1}-x_{2}\right)+\left(y_{1}-y_{2}\right)\left(y_{1}-y_{2}\right)\right] \\
&=-f g \frac{a \alpha \omega r_{12}}{\left(1+\beta r_{12}\right)^{2}}
\end{aligned}
$$

And then, we insert the double derivatives.

$$
\begin{aligned}
\frac{\nabla^{2} \Psi_{T}}{\Psi_{T}}=& 2 \alpha^{2} \omega^{2}\left(r_{1}^{2}+r_{2}^{2}\right)-4 \alpha \omega-\frac{2 a \alpha \omega r_{12}}{\left(1+\beta r_{12}\right)^{2}}+\\
& \frac{2 a}{\left(1+\beta r_{12}\right)^{2}}\left[\frac{a}{\left(1+\beta r_{12}\right)^{2}}+\frac{1}{r_{12}}-\frac{2 \beta}{1+\beta r_{12}}\right]
\end{aligned}
$$

Now, by the relation $E_L = \frac{1}{\Psi_T}H \Psi_T$ we can get the analytical expression for the *local energy*:
$$
\begin{aligned}
E_{L}=& 2 \alpha^{2} \omega^{2}\left(r_{1}^{2}+r_{2}^{2}\right)-4 \alpha \omega-\frac{2 a \alpha \omega r_{12}}{\left(1+\beta r_{12}\right)^{2}}+\\
& \frac{2 a}{\left(1+\beta r_{12}\right)^{2}}\left[\frac{a}{\left(1+\beta r_{12}\right)^{2}}+\frac{1}{r_{12}}-\frac{2 \beta}{1+\beta r_{12}}\right]+\\
& \frac{1}{2} \omega^{2}\left(r_{1}^{2}+r_{2}^{2}\right)+\frac{1}{r_{12}}
\end{aligned}
$$