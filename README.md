# gore
Graph Optimization - revisited

I hope this repo will become the rust version of the g2o repository, which is the C++ framework for global graph optimization.

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
