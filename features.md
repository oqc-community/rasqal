Whenever Munchkin is first introduced to people they usually ask "How is this different from other hybrid runtimes? It looks exactly the same.".
On the surface it operates very similarly, so this question never comes as a surprise.

Its goals and the way it views the hybrid IRs is actually quite different, but to highlight these differences requires a bit of an explainer about some of its foundational concepts and philosophies.
Giving an introduction to them will paint its features (and how it works) in a different light, so it's worth quickly explaining them before going into what it can do.

Or if you don't need convincing, feel free to jump straight to the features section.

## Core Concepts

Describing Munchkin as a symbolic-execution driven optimizing runtime, while mostly accurate, also manages to tell us very little about how it works.

A more precise description is that it's a quantum algorithm execution synthesis engine. 
It treats hybrid IRs as nothing but a blueprint for the algorithms logic, but how that algorithm is actually realized is up to it.
It may decide to recursively squash classical expressions into your circuit, or alternatively expand quantum results into classical, or squash two quantum methods together. 
As long as the codes logic ends up with the same result, how it gets there is entirely up to Munchkin to decide.
Multiple -PU machines may also be roped in to run the algorithm or power its optimization passes: QPU, GPU, CPU, HPC(U). 
Entirely depends upon what machines are nearby.

_Note: As of writing it only uses local CPU and QPU, the others will hopefully be utilized at some point in the future. Also it's not as aggressive in optimizations as this yet, early versions hold pretty closely ._

To make these decisions accurately it needs to know everything about an algorithm as it runs. 
Here's where its internal structures come into play: the graphs it builds are almost entirely constant. 
No system calls, no IO, the only variables are the ones passed in from our entry-point.
The only exception to this rule is when it's decided we need to call out to a QPU.
But as all our information is towards making these executions as optimized as possible this is an acceptable area of nondeterminism.

These nearly-constant graphs allow very strong assertions to be given around data- and control-flow, and inform us to how the circuit to be sent to the QPU should be built.
These graphs are then executed, and as we step through them we dynamically build up the queries to send to other hardware, executing when we reach a point that we absolutely need the result.

And that's pretty much it. It's a quantum algorithm generator masquerading as a runtime.

Hopefully this will have given a little insight into how its internals differ from other hybrid runtimes but also about what its goal is.

## 
