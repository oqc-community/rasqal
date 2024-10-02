## Examples

**Running examples without building Rasqal:** Install [poetry](https://python-poetry.org/) and do `poetry install` in `rasqal/examples` which will set up the venv for you. You can then run `run python examples.py` to just run the script or use your favourite IDE to debug.

If you've already built Rasqal via its build script its venv will have all the dependencies necessary so re-use that.

Note: all our examples are built using the old Q# compiler as Rasqal can exploit its fully interwoven classical LLVM instructions. 

**Examples.py** holds runnable examples of Rasqal including returned value, arguments, and custom backends. 
Source for most examples can be found in `src/tests/qsharp`. 

**Sandbox.py** runs the sandbox Q# project in `qsharp/src`. This uses the new Q# compiler so instruction set is limited.