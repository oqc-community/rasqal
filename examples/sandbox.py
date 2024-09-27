import qsharp
import os
from qsharp import init, TargetProfile
from rasqal.runtime import RasqalRunner

from utils import TracingRuntime

directory_path = os.path.dirname(os.path.abspath(__file__))


def run_sandbox():
    source_path = os.path.join(directory_path, "qsharp")

    # This initializes the static interpreter, so everything you want for QIR emission has to be set up here.
    init(project_root=source_path, target_profile=TargetProfile.Adaptive_RI)

    # There's no auto entry-point detection and this expects an expression, including any arguments.
    # So Namespace.EntryMethod(...) will then generate the QIR you want.
    sandbox = qsharp.compile("Sandbox.Main()")

    runtime = TracingRuntime()

    runner = RasqalRunner(runtime)
    results = runner.run_ll(str(sandbox))

    print(f"Sandbox results: {results}")


run_sandbox()
