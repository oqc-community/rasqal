![build status](https://github.com/oqc-community/munchkin/actions/workflows/build.yaml/badge.svg)

Munchkin is a symbolic execution quantum-classical hybrid runtime which consumes hybrid IR's such as QIR, runs 
it while performing dynamic optimizations, then calls into a provided QPU backend to run the synthesized 
circuits.

It provides hybrid computation capabilities to QPUs/tools that only have a circuit-level API available, 
as well as providing a platform for dynamic optimization/lowering algorithms.

**Note: Munchkin is still in a work-in-progress even though its core is there and usable. If you have a feature you'd like to see raise it as a request!**

### Quick-start

1. Install Munchkin in your favourite Python venv by running `pip install munchqin`. Our current testing is done with `v3.9` of Python.
2. Read the [examples](https://github.com/oqc-community/munchkin/blob/develop/examples.md).
3. (Optional) Read the [draft paper](https://github.com/oqc-community/munchkin/blob/develop/docs/Munchkin%20Draft%20v2.pdf) for a deep-dive into Munchkins concepts and data structures.

### Key Features

* Can parse and execute full spec QIR (sans big ints). This also includes most of LLVMs classical instructions.
* Multi/split-QPU execution capabilities. Each synthesized quantum circuit can be fired at a different machine depending on what features it requires to run.
* Backend QPU execution code is pure Python to allow for easy integration with existing systems.

### Experimental Features

Munchkin is also a research platform for trying out novel approaches to quantum execution, verification or analysis. 
This is loosely what we're experimenting with, or want too:

1. Quantum state compressed representation. Trading precision for linear scalability.
2. Distributed quantum execution using circuit weaving/snipping.
3. Quantum static analysis structures / circuit value prediction.

### Contributing

If you'd like to contribute your first destination will be to [build the system locally](https://github.com/oqc-community/munchkin/blob/develop/building.md).
There's also a [getting started](https://github.com/oqc-community/munchkin/blob/develop/development.md) page that covers some of the most important bits you'd need to know about the project before jumping into writing code.

After that feel free to fork the project and put up PRs with any work you would like to add.
All experimental work that isn't ready for prime time has to be disabled by default and have no impact on core execution time and stability.

Thanks for making Munchkin better than it was!

We also have a [code of conduct](https://github.com/oqc-community/munchkin/blob/develop/code_of_conduct.md) that we expect everyone to adhere too.

### Licence

This code in this repository is licensed under the BSD 3-Clause Licence.
Please see [LICENSE](https://github.com/oqc-community/munchkin/blob/develop/LICENSE) for more information.
