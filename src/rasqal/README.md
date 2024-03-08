![build status](https://github.com/oqc-community/rasqal/actions/workflows/build.yaml/badge.svg)

Rasqal is a quantum-classical hybrid runtime which runs QIR in a fully dynamic fashion, building up quantum circuits on the fly and executing them against a provided quantum backend.
It uses symbolic execution and heavily deferred execution to perform code transformations, optimizations and lowering to power the circuit synthesis.

Some of the key things this approach enables:

1. Unrestricted QIR and LLVM instructions fully interwoven. You can throw whatever form of IR you want at it and it'll process all classical bits locally (or lower them).
2. Enabling hybrid algorithms to be run on machines and tools with only a gate-level API available. This includes QASM API's if you use its simulation framework.
3. Lots of optimization potential when passed large amounts of classical context that a quantum algorithm uses to accentuate its own execution.

We also have a [full feature list and quick intro to its concepts](https://github.com/oqc-community/Rasqal/blob/develop/docs/features_and_concepts.py) as well as a [draft paper](https://github.com/oqc-community/rasqal/blob/develop/docs/Rasqal%20Draft%20v2.pdf) that covers its internals in excruciating detail.

If you have any features or ideas you'd like to see implemented feel free to raise a [feature request](https://github.com/oqc-community/Rasqal/issues/new?assignees=&labels=enhancement&projects=&template=feature_request.md&title=)!

**Note: Rasqal is still early days and the potential instruction combinations of LLVM and QIR are immense, so we won't have been able to test all of them. If you have a file which dosen't work please raise an issue with it!**

### Getting Started

1. Install Rasqal in your favourite Python venv by running `pip install rasqal`. Our current testing is done with `v3.9` of Python.
2. Read the [quick start](https://github.com/oqc-community/rasqal/blob/develop/examples.md) and look at our [Python example](https://github.com/oqc-community/Rasqal/blob/develop/docs/examples.py).
3. (Optional) Read the [paper](https://github.com/oqc-community/rasqal/blob/develop/docs/Rasqal%20Draft%20v2.pdf) for a deep-dive into Rasqals concepts and data structures.

### Contributing

If you'd like to contribute your first destination will be to [build the system locally](https://github.com/oqc-community/rasqal/blob/develop/building.md).
There's also a [getting started](https://github.com/oqc-community/rasqal/blob/develop/development.md) page that covers some of the most important bits you'd need to know about the project before jumping into writing code.

After that feel free to fork the project and put up PRs with any work you would like to add.
All experimental work that isn't ready for prime time has to be disabled by default and have no impact on core execution time and stability.

Thanks for making Rasqal better than it was!

We also have a [code of conduct](https://github.com/oqc-community/rasqal/blob/develop/code_of_conduct.md) that we expect everyone to adhere too.

### Licence

This code in this repository is licensed under the BSD 3-Clause Licence.
Please see [LICENSE](https://github.com/oqc-community/rasqal/blob/develop/LICENSE) for more information.
