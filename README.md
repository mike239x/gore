# gore
Graph Optimization - revisited

I hope this repo will become the rust version of the g2o repository, which is the C++ framework for global graph optimization.

## Background knowledge

The following is the TLDR of the paper by Rainer Kuemmerle, Giorgio Grisetti, Hauke Strasdat,
Kurt Konolige, and Wolfram Burgard - 
[g2o: A General Framework for Graph Optimization](http://ais.informatik.uni-freiburg.de/publications/papers/kuemmerle11icra.pdf)
IEEE International Conference on Robotics and Automation (ICRA), 2011.

Think about a concrete example with a car and a localization/SLAM problem.
You want to estimate the position of the car, but you can't measure car position directly.
Instead you have some measurements related to the car position. It can be IMU telling you how far you moved, some
distance measurements to some fixed points in space (f.e. GPS data), or something more complex like keypoints tracked
accros multiple camera images.

To transform this to a graph optimazition task we first need to define the graph, then define the function to be
optimized. The nodes of the graph represent the values we want to estimate, while the edges of the graph represent
measurements. Imagine a measurement that affects two positions. If we have estimates for those two positions, $x_i$ and
$x_j$, then we can compute an error $e_{ij}(x_i, x_j)$ between what the position estimates tell us and what the 
measurement tells us. Here $e_{ij}$ is a vector. Based on it we also introduce a scalar value - 
$F_{ij} = e_{ij}^T Ω_{ij} e_{ij}$. Here $Ω_{ij}$ is the "information matrix" - which will be discussed later.
Then what we want to optimize is actually $F = \sum_{i,j} e_{i,j}$.

### Information matrix

It is generally very tricky to figure out what your information matrix should be. Generally speaking, the more uncertain
your mesasurements are, the less information they preserve. TODO.

### How it works

We can approximate $e_{ij}(x_i+Δx_i, x_j+Δx_j) = e_{ij}(x+Δx)$ using the Jacobian as 
$e_{ij}(x) + J_{ij}Δx$. Then we can approximate $F_{ij}$ as 
$(e_{ij}+J_{ij}Δx)^T Ω_{ij} (e_{ij}+J_{ij}Δx) = e_{ij}^T Ω_{ij} e_{ij} + 2 e_{ij}^T Ω_{ij} J_{ij} Δx + Δx^T J_{ij}^T Ω_{ij} J_{ij} Δx = c_{ij} + 2 b_{ij} Δx + Δx^T H_{ij} Δx$.

Hence $F$ can be (locally) minimized by solving $HΔx=-2b$, where $H$ and $b$ are sums of corresponding $H_{ij}$ and
$b_{ij}$. $H$ is (similar to $Ω$) also called the information matrix of the system.

## Why use Rust

I see a few problems with the current state of g2o and want to fix those:

1. A lot of pointers waiting to become memory leaks. This is getting better
   over time, but I think it should be the very basic part of the software.
   Rust is much better in that regard, although one might get problems with
   coding a graph (I heard Rust dislikes double-linked lists).

2. Excessive code. g2o is hard to ship - you need to manually separate header
   files from everything else, you need to know you have to fetch the
   `config.h` file, you have parts of the code you don't need in your header
   files. For example the visualization and IO is something completely
   unused by me.
   The solution I see in mind is to use traits, so nothing is forsed to have
   particular IO or visualization implementation, but can have it. Rust with
   its build-in traits shines once again.

## Other suggestions

1. Make it header only. This should make building a lot easier
   (I dunno about this one though).

2. License? This is a though one for me. In theory it's a good idea to go for
   GPL, since then one would be able to use SuiteSparse. But at the same time I
   wonder if I personally want GPL license.

3. Getter name perhaps? Gore from japanise means violence, and I thought that 
   would be funny, but getting a better name would be nice.
