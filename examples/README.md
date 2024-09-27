## Examples

**Running examples without building Rasqal:** Install [poetry](https://python-poetry.org/) and do `poetry install` in `rasqal/examples` or pip install the dependencies listed in the .toml file.

If you've already built Rasqal via its build script its venv will have all the dependencies necessary so re-use that.

Note: all our examples are built using the old Q# compiler as Rasqal can exploit its fully interwoven classical LLVM instructions. 

**Examples.py** holds runnable examples 


runnable examples of many of Rasqals internal test projects showing how you set up and run things, including backend and argument definition. Shows the Q# that the QIR was generated from for each example, along with tertiary information. 
Source for most examples can be found in `src/tests/qsharp` and can be modified from there and re-built. 

**Sandbox.py** runs the sandbox Q# project in `qsharp/src`. This uses the new Q# compiler so instruction set is limited.