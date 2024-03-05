import os.path
import pathlib
import unittest
from os.path import abspath, dirname, join

from munchqin.utils import initialize_logger

from .file_utils import get_qir_path
from munchqin.simulators import fetch_qasm_runtime
from munchqin.runtime import (
    BuilderAdaptor,
    RuntimeAdaptor, MunchkinRuntime,
)


initialize_logger(os.path.join(pathlib.Path(__file__).parent.resolve(), "logs", "munchkin_output.txt"))


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


class BuilderStats:
    def __init__(self):
        self.x_count = 0
        self.y_count = 0
        self.z_count = 0
        self.cz_count = 0
        self.cx_count = 0
        self.cy_count = 0
        self.swap_count = 0
        self.reset_count = 0
        self.measure_count = 0


class BuilderMock(BuilderAdaptor):
    def __init__(self):
        self.gates = []
        self.metrics = BuilderStats()

    def cx(self, controls, target, radii):
        self.metrics.cx_count += 1
        self.gates.append(f"cx {controls} {target} {radii}")

    def cz(self, controls, target, radii):
        self.metrics.cz_count += 1
        self.gates.append(f"cz {controls} {target} {radii}")

    def cy(self, controls, target, radii):
        self.metrics.cy_count +=  1
        self.gates.append(f"cy {controls} {target} {radii}")

    def x(self, qubit, radii):
        self.metrics.x_count += 1
        self.gates.append(f"x {qubit} {radii}")

    def y(self, qubit, radii):
        self.metrics.y_count += 1
        self.gates.append(f"y {qubit} {radii}")

    def z(self, qubit, radii):
        self.metrics.z_count += 1
        self.gates.append(f"z {qubit} {radii}")

    def swap(self, qubit1, qubit2):
        self.metrics.swap_count += 1
        self.gates.append(f"swap {qubit1} {qubit2}")

    def reset(self, qubit):
        self.metrics.reset_count += 1
        self.gates.append(f"reset {qubit}")

    def measure(self, qubit):
        self.metrics.measure_count += 1
        self.gates.append(f"measure {qubit}")

    def clear(self):
        if any(self.gates):
            self.gates.append("clear")


class RuntimeMock(RuntimeAdaptor):
    def __init__(self):
        self.executed = []

    def execute(self, builder: BuilderMock):
        self.executed.append(builder)
        return dict()

    def create_builder(self) -> BuilderAdaptor:
        return BuilderMock()

    def has_features(self, required_features):
        return True

    @property
    def builder_instructions(self):
        builder = next(iter(self.executed), None)
        return builder.gates if builder is not None else None


def fetch_mock_runtime():
    runtime = RuntimeMock()
    return runtime, MunchkinRuntime(runtime)


def build_architecture(num_qubits):
    return [(i % num_qubits, (i + 1) % num_qubits) for i in range(num_qubits)]


class TestMunchkin:
    def test_simulated_qaoa(self):
        qir = fetch_project_ll("qaoa")
        runtime = fetch_qasm_runtime(20)
        results = runtime.run(qir)

        # This prints its result, not returns.
        assert results is None

    def test_qaoa(self):
        qir = fetch_project_ll("qaoa")
        mock, runtime = fetch_mock_runtime()
        runtime.trace_runtime().trace_projections()
        runtime.run(qir)

        for stats in [builder.metrics for builder in mock.executed]:
            stats: BuilderStats
            assert stats.x_count == 30
            assert stats.y_count == 6
            assert stats.z_count == 111
            assert stats.cx_count == 300
            assert stats.cy_count == 0
            assert stats.cz_count == 0
            assert stats.measure_count == 6
            assert stats.reset_count == 5
            assert stats.swap_count == 0

    def test_oracle_gen(self):
        qir = fetch_project_ll("oracle-generator")
        mock, runtime = fetch_mock_runtime()
        runtime.trace_runtime().trace_projections()
        runtime.run(qir)

        assert mock.executed[0].gates == ['measure 0', 'measure 1', 'measure 2']
        assert mock.executed[1].gates == ['x 2 3.141592653589793', 'measure 0', 'measure 1', 'measure 2']
        assert mock.executed[2].gates == ['x 1 3.141592653589793', 'measure 0', 'measure 1', 'measure 2']
        assert mock.executed[3].gates == ['x 1 3.141592653589793', 'x 2 3.141592653589793', 'measure 0', 'measure 1', 'measure 2']
        assert mock.executed[4].gates == ['x 0 3.141592653589793', 'measure 0', 'measure 1', 'measure 2']
        assert mock.executed[5].gates == ['x 0 3.141592653589793', 'x 2 3.141592653589793', 'measure 0', 'measure 1', 'measure 2']
        assert mock.executed[6].gates == ['x 0 3.141592653589793', 'x 1 3.141592653589793', 'measure 0', 'measure 1', 'measure 2']
        assert mock.executed[7].gates == ['x 0 3.141592653589793', 'x 1 3.141592653589793', 'x 2 3.141592653589793', 'measure 0', 'measure 1', 'measure 2']

    def test_minified_generator(self):
        qir = fetch_project_ll("minified-oracle-generator")

        mock, runtime = fetch_mock_runtime()
        runtime.run(qir, [True])
        assert mock.builder_instructions == [
            'x 0 3.141592653589793',
            'measure 0'
        ]

        mock, runtime = fetch_mock_runtime()
        runtime.run(qir, [False])
        assert mock.builder_instructions == [
            'measure 0'
        ]

    def test_simplified_generator(self):
        qir = fetch_project_ll("simplified-oracle-generator")
        mock, runtime = fetch_mock_runtime()
        runtime.run(qir)

        assert mock.executed[0].gates == ['measure 0', 'measure 1', 'measure 2']
        assert mock.executed[1].gates == ['x 2 3.141592653589793', 'measure 0', 'measure 1', 'measure 2']
        assert mock.executed[2].gates == ['x 1 3.141592653589793', 'measure 0', 'measure 1', 'measure 2']
        assert mock.executed[3].gates == ['x 1 3.141592653589793', 'x 2 3.141592653589793', 'measure 0', 'measure 1', 'measure 2']
        assert mock.executed[4].gates == ['x 0 3.141592653589793', 'measure 0', 'measure 1', 'measure 2']
        assert mock.executed[5].gates == ['x 0 3.141592653589793', 'x 2 3.141592653589793', 'measure 0', 'measure 1', 'measure 2']
        assert mock.executed[6].gates == ['x 0 3.141592653589793', 'x 1 3.141592653589793', 'measure 0', 'measure 1', 'measure 2']
        assert mock.executed[7].gates == ['x 0 3.141592653589793', 'x 1 3.141592653589793', 'x 2 3.141592653589793', 'measure 0', 'measure 1', 'measure 2']

    @unittest.skip("Need to defer measure into classical results.")
    def test_deferred_classical_expression(self):
        qir = fetch_project_ll("def-classical-expression")
        mock, runtime = fetch_mock_runtime()
        runtime.run(qir)

    def test_routed_bell_psi_plus(self):
        mock, runtime = fetch_mock_runtime()
        runtime.apply_routing(build_architecture(4))
        runtime.run(get_qir_path("bell_psi_plus.ll"))

        assert mock.builder_instructions == [
            "z 3 3.141592653589793",
            "y 3 1.5707963267948966",
            "cx [3] 0 3.141592653589793",
            "measure 3",
            "measure 0",
        ]

    def test_parser_bell_psi_plus(self):
        mock, runtime = fetch_mock_runtime()
        runtime.run(get_qir_path("bell_psi_plus.ll"))

        assert mock.builder_instructions == [
            "z 0 3.141592653589793",
            "y 0 1.5707963267948966",
            "cx [0] 1 3.141592653589793",
            "measure 0",
            "measure 1",
        ]

    def test_parser_bell_psi_minus(self):
        mock, runtime = fetch_mock_runtime()
        runtime.run(get_qir_path("bell_psi_minus.ll"))

        assert mock.builder_instructions == [
            "x 0 3.141592653589793",
            "z 0 3.141592653589793",
            "y 0 1.5707963267948966",
            "cx [0] 1 3.141592653589793",
            "measure 0",
            "measure 1",
        ]

    def test_parser_bell_theta_plus(self):
        mock, runtime = fetch_mock_runtime()
        runtime.run(get_qir_path("bell_theta_plus.ll"))

        assert mock.builder_instructions == [
            "x 1 3.141592653589793",
            "z 0 3.141592653589793",
            "y 0 1.5707963267948966",
            "cx [0] 1 3.141592653589793",
            "measure 0",
            "measure 1",
        ]

    def test_parser_bell_theta_minus(self):
        mock, runtime = fetch_mock_runtime()
        runtime.run(get_qir_path("bell_theta_minus.ll"))

        assert mock.builder_instructions == [
            "x 1 3.141592653589793",
            "x 0 3.141592653589793",
            "z 0 3.141592653589793",
            "y 0 1.5707963267948966",
            "cx [0] 1 3.141592653589793",
            "measure 0",
            "measure 1",
        ]
