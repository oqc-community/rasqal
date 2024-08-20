## Examples

All dependencies to run the examples are installed into the Rasqal venv if you are building normally.
If you can't run the full build, use `./bulid.ps1 -t "initialize-examples"` to install a Rasqal versio from pypi as a replacement.

All files can be run directly.

**Examples.py** holds examples of how to use Rasqals Python APIs to run QIR.

**Sandbox.py** runs the sandbox Q# project in `qsharp/src`.
Modify the project as you need and then run the Python file to see how Rasqal reacts.
Currently restricted to adaptive profile QIR.