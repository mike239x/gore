# gore
Graph Optimization - revisited

I hope this repo will become the new version of the g2o repository.

I see a few problems with the current state of g2o and want to fix those:

1. A lot of pointers waiting to become memory leaks. This is getting better
   over time, but I think it should be the very basic part of the software.
2. Excessive code. g2o is hard to ship - you need to manually separate header
   files from everything else, you need to know you have to fetch the
   `config.h` file, you have parts of the code you don't need in your header files.
   For example the visualization and IO is something completely unused by me.
3. Building. Not the most straightforward building. Executables and tests are
   generated inside of the main build (but why?); a lot of types get compiled
   into singular libs which should be linked against even if you want to use
   just 1 or 2 types out of them all. Also, requires you to know cmake, which
   is perhaps a good thing, but still, it's a restriction.

Now, my suggestions:

1. Use shared and/or unique pointers
2. Make it header only. This instantly solves the problem of building.
3. Use traits for drawing and IO - this way drawing / IO can be plugged at any
   moment you want by just including an extra header file. Maybe also make the
   vertices into traits? This way one can literally make vertex out of anything.

Also, here are some other organizatorical suggestions:

1. Use C++17. Yes I am for moving things forward; and sticking to C++11 feels
   like a waste.
2. Something something cmake version?
3. License? This is a though one for me. In theory it's a good idea to go for
   GPL, since then one would be able to use SuiteSparse. But at the same time I
   wonder if I personally want GPL license.
