# non_negative_DP
implement some of the non-negative DP distributions from this paper https://arxiv.org/abs/2110.08177

# Truncated Double Geometric (Discrete Laplace)
[Definition of Truncated Double Geometric from paper]

The process of drawing a sample from a Truncated Double Geometric will be done by sampling from a series of distributions
1. We will assume access to samples from a Bernoulli distribiton as provided by the `rand` crate
2. We will use this to implement sampling from a Geometric distribution
3. We will use the geometric distribution to implement sampling from a double geometric distribution
4. We will use rejection sampleing from a double geometric to sample from a truncated double geometric.

## Geometric Distribuiton
We take the Geometric Distribution to be the probability distribution of the number of failures of Bernoulli trials before the first success, supported on the set $\{0,1,2,...\}.  Let $0 < p \leq 1$ be the success probability of the Bernoulli trials.  The mean of the geometric is $\mu = \frac{1-p}{p}$ and variance is $\sigma^2 = \frac{1-p}{p^2}$.

## Double Geometric Distribution
We use the following from this [book](https://www.researchgate.net/publication/258697410_The_Laplace_Distribution_and_Generalizations) page 159.

A double geometric distribution has probability function
$$f(k)=c(s)e^{|k-\theta|/s},k=0,\pm 1, \pm 2,...$$
where $\theta$ is an integer, $s$ is a positive real number, and $c(s)$ is a normalizing constant.  It is a property that a double geometric random variable, $Y$, with the above probability function can be represented as
$Y=\theta + X_1 - X_2$
where $X_1$ and $X_2$ are iid geometric variables with success probability $p = 1 - e^{-1/s}$.


The variance of a double geometric is the sum of the variances of the two independent geometrics, $X_1$ and $X_2$, so is $2 (\frac{1-p}{p^2})$

## Truncated Double Geometric Distribution
A truncated double geometric distribution, as considered in section 3.2 of this [paper](https://arxiv.org/pdf/2110.08177.pdf), has the following probability function
$$f(x) = A e^{\varepsilon |n-x|}; x\in \{0,...,2n\}$$
for some normalizing constant $0< A< 1$ and some $n \in \N$.  Here we can see that this is like the double geometric distribution except that it has bounded support. We can sample from it by sampling from the double geometric with rejection if the sample lies outside the support set.

# Tests

## Tests for Geometric Distribution
### Test outliers with CDF
The CDF of a geometric is $1-(1-p)^{\lfloor x \rfloor -1}$ for $x\geq 0$. If we consider the smallest integer $x$ such 99.9% of the probability mass is less than $x$ $0.999 \leq 1-(1-p)^{ x -1}$ we can solve for $x$ as $\lfloor \frac{\ln(0.001)}{\ln(1-p)}\rfloor $.  We create a test that draws 100 random samples and checks that they are all less than this bound.  This test will fail randomly 1% of the time.

### Anti-concentration test with CDF
The above test made sure we were not getting too large of values from the geometric; this test will ensure we aren't getting too small of values either.  The probability a sample is greater than the mean is 50%. Draw 100 random samples and check that at least $m$ of them are greater than the mean. This will fail with probability $f(m)$ [TODO exact details].


### Tests using Chebyshev's inequality
We will use Chebyshev's inequality to setup a basic test that the distribution we are sampling from follows the expected behavior $\Pr[ |X - \mu| \geq t] \leq \frac{\sigma^2}{t^2}$.  We will create a test

## Tests for Double Geometric

## Tests for Truncated Double Geometric
### Test using Hoeffding's inequality (a Chernoff bound)
Hoffding's inequality [sitation]() states that for any independent random variables $Z_1, ..., Z_m$ where each is bounded, $Z_i \in [\ell_i, u_i]$,
$$\Pr \left [ \left | \frac{1}{m} \sum_{i=1}^{m}Z_i - \frac{1}{m} \sum_{i=1}^{m} E[Z_i] \right | \geq t  \right] \leq 2 e^{\frac{-2m^2t^2}{\sum_{i=1}^{m} (u_i-\ell_i)^2}}$$
if each $Z_i \in [\ell , u]$ the exponent is $\frac{-2mt^2}{(u-\ell)^2}$.  This matches well the case of our truncated double geometric since it is bounded. We will implement two versions of this test
1. All samples with the same parameters, such that we sample $m=1000$ $Z_i$'s and check that the average of these samples is no further than $t$ from the expected value of the distribution.  If it is futher than $t$, the test will fail and we will set $t$ using the probability bound above such that it fails randomly ~1% of the time.
2. We will draw $m$ samples all from different truncated double geometric distributions with randomly asigned parameters. The test will check that the average of these samples is no further than $t$ from the average of the expected values of the distributions.  Again $t$ will be set such that the test fails randomly ~1% of the time.
