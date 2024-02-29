![build status](https://github.com/oqc-community/munchkin/actions/workflows/build.yaml/badge.svg)

Munchkin is a symbolic execution quantum-classical hybrid runtime which consumes hybrid IR's such as QIR, runs 
it while performing dynamic optimizations, then calls into a provided QPU backend to run the synthesized 
circuits.

It provides hybrid computation capabilities to QPUs/tools that only have a circuit-level API available, 
as well as providing a platform for dynamic optimization/lowering algorithms.

**Note: Munchkin is still heavily work-in-progress even though its features are already useful. We'd love to hear what sort of features and ideas you think would fit in!**

### Quick-start

1. Install Munchkin in your favourite Python venv by running `pip install munchqin`.
2. Read the [examples](https://github.com/oqc-community/munchkin/blob/develop/examples.md).
3. (Optional) Read the [draft paper](https://github.com/oqc-community/munchkin/blob/develop/docs/Munchkin%20Draft%20v2.pdf) for a deep-dive into Munchkins concepts and data structures.

### Contributing

If you'd like to contribute your first destination will be to [build the system locally](https://github.com/oqc-community/munchkin/blob/develop/building.md).
There's a [getting started](https://github.com/oqc-community/munchkin/blob/develop/development.md) page that covers some of the most important bits you'd need to know about the project before jumping into writing code.

After that feel free to fork the project and put up PRs with any work you would like to add. All experimental work that isn't ready for prime time has to be disabled by default and have no impact on existing runtime or features when it is.

We may not accept all PRs even if we appreciate any work people would like to add. If you really want to add something but may not be sure it'll fit, please just raise an issue as a feature request.
We'll review it and either give the green light or recommended changes, potentially even advising a secondary tool that would fit better.

Thanks for making Munchkin better than it was!

We also have a [code of conduct](https://github.com/oqc-community/munchkin/blob/develop/code_of_conduct.md) that we expect everyone to adhere too.

### Licence

This code in this repository is licensed under the BSD 3-Clause Licence.
Please see [LICENSE](https://github.com/oqc-community/munchkin/blob/develop/LICENSE) for more information.
