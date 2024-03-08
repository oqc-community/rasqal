If you want to jump right in, here are some [full working examples](https://github.com/oqc-community/rasqal/blob/develop/docs/examples.py)
of running .ll files and building various backends for integration.

Everything this document talks about can be seen in action there.

### Getting started

To run Rasqal you'll need a QIR file, whether in its human-readable .ll form or bitcode. 
We have some pre-built QIR that we use for tests (`src/tests/qsharp`) that you can use and modify if needed.

For our first example we're going to use the default simulator backend, so will not delve into the details required to add your own.

Since we're not providing a custom backend our Python to run Rasqal is relatively simple. 
If your QIR has no return value or arguments, this is how you call it:
```python
from rasqal.simulators import fetch_qasm_runner

# Create a QASM simulation backend with 20 qubits available.
runtime = fetch_qasm_runner(20)
runtime.run("path_to_ll_file")
```

If your QIR entry-point has an argument, lets say a string and int, this is what you need:
```python
# ...
runtime.run("path_to_ll_file", ["arg_1", 5])
```

And finally if your call also returns an argument, it would look like this:
```python
# ...
results = runtime.run("path_to_ll_file", ["arg_1", 5])
assert results == 42
```

This then runs your QIR locally and fires off any quantum fragments to the backend simulator.

Note: while Rasqal accepts full-spec QIR and most classical LLVM instructions it doesn't allow system calls such as I/O or sockets.
Such calls will simply be ignored if used in core logic will cause an exception.

If you want to use such things they have to be passed in as arguments or done outside Rasqals execution loop.

### Backends

If you want to intercept Rasqals quantum executions and redirect it for your own uses you will need to use two additional objects - the `BuilderAdaptor` and `RuntimeAdaptor`.

Both of these are shim API's whose methods Rasqal expects to exist on whatever Python object you pass it.

`BuilderAdaptor` is a gate- and instruction-level API that will be called to build your circuit after Rasqal knows what's needed and wants to get a result.
It's called sequentially with all the gates and instructions that are going to be used with the incoming execution.

`RuntimeAdaptor` is the execution API. It will get called with the builder it wants to be executed and will wait for a result. Results must be in a bit string results distribution:

```python
{
    "010": 250,
    "111": 182,
    ...
}
```

Both API's can be found [here](https://github.com/oqc-community/rasqal/blob/develop/src/rasqal/rasqal/adaptors.py).

After you have both a Runtime and Builder you then can use them as a backend for execution by passing them to a Rasqal runtime:
```python
from rasqal.adaptors import BuilderAdaptor, RuntimeAdaptor
from rasqal.runtime import RasqalRunner

class CustomBuilder(BuilderAdaptor):
    ...

class CustomRuntime(RuntimeAdaptor):
    def create_builder(self) -> BuilderAdaptor:
        return CustomBuilder()
    
    ...

runtime = RasqalRunner(CustomRuntime())
runtime.run("path_to_qir")
```

If you have multiple backends you can just pass them in as a list to the constructor:
```python
runtime = RasqalRunner([QPURuntime(), SimulatorRuntime()])
```
When quantum code needs to be executed they will, in turn, be asked whether they can run it. 
The first runtime which answers yes will then be used for that execution.

The `fetch_qasm_runner` method we used earlier is simply a wrapper which loads our QASM builder and runtime in.

With that, our custom classes will now be called when a quantum execution is needed, well if we put in the various methods anyway.

If you'd like a template, our [QASM backends](https://github.com/oqc-community/rasqal/blob/develop/src/rasqal/rasqal/simulators.py) can provide one.

### Debugging

Symbolic execution engines are complicated by their nature so debugging it can be a little tricky unless you understand it's output.

The runtime itself exposes various tracing mechanisms that you can activate for a run:
```python
from rasqal.runtime import RasqalRunner

runtime = RasqalRunner(...)

# Prints out every step the runtime takes.
runtime.trace_runtime()

# Prints out all the information of a quantum projections execution and analysis.
# (The thing which compresses and then builds the circuit and executes it via the Python objects)
runtime.trace_projections()

# Outputs the entire graph that we're going to run.
runtime.trace_graphs()
```

By default, these are all printed to the console. You can initialize Rasqals file logging mechanism by calling `initialize_logger` with a file path.
This is recommended if you enable traces as it produces a _lot_ of output. 

Traces are not lightweight and should only be used for debugging or informational purposes. 
They should not be left on in a live system.