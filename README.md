[![PyPi Deployment](https://github.com/oqc-community/rasqal/actions/workflows/deploy-wheels.yml/badge.svg?event=release)](https://github.com/oqc-community/rasqal/actions/workflows/deploy-wheels.yml)

<img src="https://github.com/oqc-community/rasqal/blob/develop/logo.png#gh-light-mode-only" align="right" width="160px">
<img src="https://github.com/oqc-community/rasqal/blob/develop/logo_mono.png#gh-dark-mode-only" align="right" width="160px"/>

Rasqal is a quantum-classical solver runtime that takes heavy inspiration from static analysis tools and SAT solvers to power optimization, transformation and circuit splice/weaving.
Its internal structures and concepts are also evolving towards a more high-level abstract representation of hybrid algorithms, so automated tools can process them better and potentially use such models to help uninitiatied developers get an intuitive understanding of quantum computing.

The details about its various ideas and components can be found in the [papers](https://github.com/oqc-community/rasqal/tree/develop/docs/papers) folder, while a quick introduction of them and current capabilities can be found [here](https://github.com/oqc-community/rasqal/blob/develop/docs/features_and_concepts.md).

If you have any features or ideas you'd like to see implemented feel free to raise a [feature request](https://github.com/oqc-community/Rasqal/issues/new?assignees=&labels=enhancement&projects=&template=feature_request.md&title=)!

**Note: Rasqal is still early days and the potential instruction combinations of LLVM and QIR are immense, so we won't have been able to test all of them. If you have a file which dosen't work please raise an issue with it!**

### Getting Started

1. Install Rasqal in your favourite Python venv by running `pip install rasqal`. Our current testing is done with `v3.10` of Python.
2. Read the [quick start](https://github.com/oqc-community/rasqal/blob/develop/docs/quick_start.md) and look at our [examples](https://github.com/oqc-community/Rasqal/blob/develop/examples/examples.py).
3. (Optional) Read the [paper](https://github.com/oqc-community/rasqal/blob/develop/docs/papers/Rasqal%20Draft%20v3.pdf) for a deep-dive into Rasqals concepts and data structures.

### Contributing

If you'd like to contribute your first destination will be to [build the system locally](https://github.com/oqc-community/rasqal/blob/develop/docs/building.md).
There's also a [getting started](https://github.com/oqc-community/rasqal/blob/develop/docs/development.md) page that covers some of the most important bits you'd need to know about the project before jumping into writing code.

After that feel free to fork the project and put up PRs with any work you would like to add.
All experimental work that isn't ready for prime time has to be disabled by default and have no impact on core execution time and stability.

Thanks for making Rasqal better than it was!

We also have a [code of conduct](https://github.com/oqc-community/rasqal/blob/develop/code_of_conduct.md) that we expect everyone to adhere too.

### Licence

This code in this repository is licensed under the BSD 3-Clause Licence.
Please see [LICENSE](https://github.com/oqc-community/rasqal/blob/develop/LICENSE) for more information.
