from os.path import abspath, dirname, join

from .file_utils import get_qir_path
from munchqin.simulators import fetch_qasm_runtime
from munchqin.runtime import (
    BuilderAdaptor,
    RuntimeAdaptor, MunchkinRuntime,
)

def fetch_project_ll(proj_name: str):
    """Return a Munchkin test file for processing via the Python APIs."""
    return abspath(
        join(
            dirname(__file__),
            "qsharp",
            proj_name,
            "qir",
            f"{proj_name}.ll",
        )
    )

class BuilderMock(BuilderAdaptor):
    def __init__(self):
        self.gates = []

    def cx(self, controls, target, radii):
        self.gates.append(f"cx {controls} {target} {radii}")

    def cz(self, controls, target, radii):
        self.gates.append(f"cz {controls} {target} {radii}")

    def cy(self, controls, target, radii):
        self.gates.append(f"cy {controls} {target} {radii}")

    def x(self, qubit, radii):
        self.gates.append(f"x {qubit} {radii}")

    def y(self, qubit, radii):
        self.gates.append(f"y {qubit} {radii}")

    def z(self, qubit, radii):
        self.gates.append(f"z {qubit} {radii}")

    def swap(self, qubit1, qubit2):
        self.gates.append(f"swap {qubit1} {qubit2}")

    def reset(self, qubit):
        self.gates.append(f"reset {qubit}")

    def measure(self, qubit):
        self.gates.append(f"measure {qubit}")

    def clear(self):
        if any(self.gates):
            self.gates.append("clear")


class RuntimeMock(RuntimeAdaptor):
    def __init__(self):
        self.executed = []

    def execute(self, builder: BuilderMock):
        self.executed = builder.gates
        return dict()


def fetch_mock_runtime():
    return MunchkinRuntime(BuilderMock(), RuntimeMock())

class TestMunchkin:
    def test_qaoa(self):
        qir = fetch_project_ll("qaoa")
        runtime = fetch_qasm_runtime(20)
        results = runtime.run(qir)

        # This prints its result, not returns.
        assert results is None

    def test_oracle_gen(self):
        qir = fetch_project_ll("oracle-generator")
        runtime = fetch_qasm_runtime(20)
        results = runtime.run(qir)

        assert results is None

    def test_minified_generator(self):
        qir = fetch_project_ll("minified-oracle-generator")
        runtime = fetch_qasm_runtime(20)
        results = runtime.run(qir, [True])

        assert results is None

    def test_parser_bell_psi_plus(self):
        runtime = fetch_mock_runtime()
        runtime.run(get_qir_path("bell_psi_plus.ll"))

        assert runtime.builder.gates == [
            "z 0 3.141592653589793",
            "y 0 1.5707963267948966",
            "cx [0] 1 3.141592653589793",
            "measure 0",
            "measure 1",
        ]

    def test_parser_bell_psi_minus(self):
        runtime = fetch_mock_runtime()
        runtime.run(get_qir_path("bell_psi_minus.ll"))

        assert runtime.builder.gates == [
            "x 0 3.141592653589793",
            "z 0 3.141592653589793",
            "y 0 1.5707963267948966",
            "cx [0] 1 3.141592653589793",
            "measure 0",
            "measure 1",
        ]

    def test_parser_bell_theta_plus(self):
        runtime = fetch_mock_runtime()
        runtime.run(get_qir_path("bell_theta_plus.ll"))

        assert runtime.builder.gates == [
            "x 1 3.141592653589793",
            "z 0 3.141592653589793",
            "y 0 1.5707963267948966",
            "cx [0] 1 3.141592653589793",
            "measure 0",
            "measure 1",
        ]

    def test_parser_bell_theta_minus(self):
        runtime = fetch_mock_runtime()
        runtime.run(get_qir_path("bell_theta_minus.ll"))

        assert runtime.builder.gates == [
            "x 1 3.141592653589793",
            "x 0 3.141592653589793",
            "z 0 3.141592653589793",
            "y 0 1.5707963267948966",
            "cx [0] 1 3.141592653589793",
            "measure 0",
            "measure 1",
        ]
