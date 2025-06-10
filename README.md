
<div align="center">
<img src="https://github.com/oqc-community/rasqal/blob/develop/logo.png#gh-light-mode-only" width="200px">
<img src="https://github.com/oqc-community/rasqal/blob/develop/logo_mono.png#gh-dark-mode-only" width="200px"/>
</div>

[![PyPi Deployment](https://github.com/oqc-community/rasqal/actions/workflows/deploy-wheels.yml/badge.svg?event=release)](https://github.com/oqc-community/rasqal/actions/workflows/deploy-wheels.yml)

-------------

## Projects

**Rasqal** is a dynamic quantum hybrid runtime with one goal: to statically solve as much of the quantum program as possible before sending what remains to a quantum computer.
It's heavily inspired by SAT solvers and interpreters, mixing techniques from both together to dynamically execute, analyze, and solve the IR it gets given.

**Forq** is an experimental, highly-scalable static analysis data structure for predicting quantum states. 
It sacrifices absolute precision for scalability and when states get too complex it condenses them, causing the bitstring results to become a predicted range of probabilities rather than a definite answer.
When stable, will be available for testing via Rasqals experimental options.

Any papers written about them can found [here](https://github.com/oqc-community/rasqal/tree/develop/docs/papers), and act as a technical deep-dive into their concepts.

## Quick Start

Install Rasqal in your favourite Python venv by running `pip install rasqal`. Our current testing is done with `v3.10` of Python.

To run a QIR/LLVM file with a default QASM runner as a backend:
```python
from rasqal.simulators import fetch_qasm_runner

# Create a QASM simulation backend with 20 qubits available.
runtime = fetch_qasm_runner(20)
runtime.run("path_to_ll_file")
```

If your entry-point has arguments that map to Python primitives, you can use them like this:
```python
from rasqal.simulators import fetch_qasm_runner

# Create a QASM simulation backend with 20 qubits available.
runtime = fetch_qasm_runner(20)
runtime.run("path_to_ll_file", ["squirrel", 42])
```

Any results from the IR are just returned like a normal Python method as long as mapping succeeds:
```python
from rasqal.simulators import fetch_qasm_runner

# Create a QASM simulation backend with 20 qubits available.
runtime = fetch_qasm_runner(20)
results = runtime.run("path_to_ll_file", ["squirrel", 42])
```

For more complex scenarios check out our [quick start](https://github.com/oqc-community/rasqal/blob/develop/docs/quick_start.md), [examples](https://github.com/oqc-community/Rasqal/blob/develop/examples/examples.py), or [tests](https://github.com/oqc-community/rasqal/tree/develop/src/tests).

## Contributing

To start contributing you can either put up PRs with additional features and fixes, which will require you first to [build the system locally](https://github.com/oqc-community/rasqal/blob/develop/docs/building.md), or raise bugs, improvements and feature requests via the repositories [Issues](https://github.com/oqc-community/rasqal/issues) tab.

## Licence

This code in this repository is licensed under the BSD 3-Clause Licence.
Please see [LICENSE](https://github.com/oqc-community/rasqal/blob/develop/LICENSE) for more information.
