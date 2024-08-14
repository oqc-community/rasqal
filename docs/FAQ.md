# FAQ

> After reading this it seems like any other hybrid runtime. What makes it different?
 
Rasqal shares more DNA with formal verification tools and [model checkers](https://en.wikipedia.org/wiki/Model_checking) than it does with runtimes even though superficially it does, indeed, run your code.
But it doesn't view it as running, in fact it's just generating a quantum equation from all the information you've given it, in the form it deduces will give you the best results.

It uses all the classical information provided to provide the constraints that the algorithm is running within, as well as analysing the evolving quantum state as much as it can.
Many of the classical constraints evaluation comes from existing ideas, but it also tries to do the same to the quantum circuit it's building (within reason).

Many of its systems are currently in their infancy so may not be immediately noticeable, but in time they will.

Rasqal works with other runtimes and sits at the exact point where they have generated the hybrid IR they consider encapsulates, fully, the algorithm they are trying to run. 
Rasqal then takes this output and crunches it down even further.