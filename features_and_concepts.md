Whenever Munchkin is first introduced to people they usually ask "How is this different from other hybrid runtimes? It looks exactly the same!".
On the surface it operates very similarly, so this question isn't that suprising.

Its goals and the way it views the hybrid IRs is actually quite different, but to highlight these differences requires a bit of an explainer about some of its foundational concepts and philosophies.
Giving an introduction to them will paint its features (and how it works) in a different light, so it's worth quickly explaining them before going into what it can do.

Or if you don't need convincing, feel free to jump straight to the features section.

## Foundational Concepts

Describing Munchkin as a symbolic-execution driven optimizing runtime, while accurate from a high level, also manages to tell us very little about how it works.

A more precise description is that it's a quantum algorithm synthesis engine. 
It treats hybrid IRs as nothing but a blueprint for the algorithms logic, but how that algorithm is actually realized is up to it.
It may decide to recursively squash classical expressions into your circuit, alternatively expand quantum results into classical, or squash two quantum methods together. 
As long as the codes logic ends up with the same result, how it gets there is entirely up to Munchkin to decide.
Multiple -PU machines may also be roped in to run the algorithm or power its optimization passes: QPU, GPU, CPU, HPC(U). 
Entirely depends upon what machines are nearby.

_Note: As of writing it only uses local CPU and QPU, GPU/HPC support will come later. 
The decision to do many of these transforms is also 'no' right now, since they aren't in or have a rather specific trigger condition._

To make these decisions accurately it needs to know everything about an algorithm as it runs. 
Here's where its internal structures come into play: the graphs it builds are almost entirely constant. 
No system calls, no IO, the only variables are the ones passed in from our entry-point.
The only exception to this rule is when it's decided we need to call out to a QPU or other external hardware.
But as all our information is towards making these executions as optimized as possible this is an acceptable area of nondeterminism.

These nearly-constant graphs allow very strong assertions to be given around data- and control-flow, and inform us to how the circuit to be sent to the QPU should be built.
The graphs are then executed, and as we step through them we dynamically build up the queries to send to other hardware, executing when we reach a point that we absolutely need the result.

Munchkin doesn't execute QIR in a traditional sense, it uses it as a schema for encoding the logic that a hybrid algorithm uses. 
Sometimes it will run very close to the QIR as-written because it's simple enough to not need transformation.
In early versions this might happen more often than not, but this is not its function.
In fact the less the execution run looks like the incoming QIR the better, because it means we've been able to do a lot of optimization and prediction.

And that's it! The introduction to its internals and its goals. 
Hopefully this has given some insight into how it differs to other hybrid runtimes (it's barely a runtime) and will help give some nuance to how some of its features actually work.

If you are interested in a more thorough breakdown of its internals and concepts it has [a paper](https://github.com/oqc-community/munchkin/blob/develop/docs/Munchkin%20Draft%20v2.pdf) which goes into them in detail.

## Features

Currently available:

1. Full-spec QIR support [1] including the majority of classical LLVM instructions. Any profile which inherits off the base spec is by implicitly supported.
2. Variational algorithm capabilities via entry-point arguments and return values.
3. Multi-QPU execution capabilities - the quantum parts of an algorithm will only run on machines capable of supporting it. 
   1. This will also support parallel and distributed QPU execution in the future.
4. Ability to route hybrid circuits. Initially supported by Tket [2].
5. Classical and quantum instructions can be fully interwoven including data- and control-flow.
6. Support for more traditional constructs such as logging and exceptions. These in the future could be lowered into the hardware.

The combination of these means that even if a QPU doesn't have built-in hybrid instruction support you can use Munchkin to execute hybrid code against it.
All it needs is gate-level operation support. 

We also have a QASM builder that is used to power our simulators, but this could also be used for integrations with anything that has a QASM API.

[1] Big int is the one exception here, this isn't supported. 
There are also some instructions we silently ignore because they have no impact on how Munchkin views the world, such as qubit reference counting.

[2] The wording here is rather precise as we don't route the QIR, we route the circuit we generate just before its sent to the QPU.
Routing fully interwoven instructions is very tricky, so we purposefully only do it after circuit synthesis.

Soon to be implemented:

1. Automatically lowering logic into the hardware if support is available, such as conditionals, loops and exceptions.
2. Aggressive classical operation deference and folding. Currently if you do a measure and immediately do classical operations on the result then it'll execute the circuit built up to that point.
We want to also defer the classical operations until we find a point where we _absolutely_ need to execute.
3. More required feature metadata to be passed to the backend. Circuit size, included gates, far more information about what's being run. 
4. Predicted execution plans. Allows tools to pass in QIR and get back precise information about each quantum execution: what circuit, values, features it requires etc.
This will allow tools sitting in front of Munchkin to tailor their own optimization passes.

5. LLVM-15+.

In the future:

1. Quantum state analysis structures for performing indepth static analysis as we go. This powers many other features.
2. Quantum fragment simulation. Finding points in a circuit that if simulated/predicted allow for better optimization or distributed processing.
3. Using our analysis tools and splice/weaving techniques to split up and run large quantum circuits across multiple smaller machines.

These are our highly experimental features that will be continuously worked on alongside more tangential improvements.
While being built they will only be enabled by experimental feature flags.

Please see [our examples](https://github.com/oqc-community/munchkin/blob/develop/examples.md) for the sorts of code you could send to Munchkin as well as what it returns.