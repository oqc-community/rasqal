# SPDX-License-Identifier: BSD-3-Clause
# Copyright (c) 2024 Oxford Quantum Circuits Ltd

from typing import Dict

from pykin import BuilderAdaptor, RuntimeAdaptor
from qiskit.providers.models import QasmBackendConfiguration

from qiskit import QiskitError, QuantumCircuit, transpile
from qiskit_aer import AerSimulator

from pykin.runtime import MunchkinRuntime


def fetch_qasm_runtime(qubit_count=30):
    return MunchkinRuntime(QASMBuilder(qubit_count), QASMRuntime())


class QASMBuilder(BuilderAdaptor):
    def __init__(self, qubit_count: int):
        super().__init__()
        self.circuit = QuantumCircuit(qubit_count, qubit_count)
        self.shot_count = 1024
        self.bit_count = 0

    def cx(self, controls, target, theta):
        self.circuit.crx(theta, controls, target)

    def cz(self, controls, target, theta):
        self.circuit.crx(theta, controls, target)

    def cy(self, controls, target, theta):
        self.circuit.cry(theta, controls, target)

    def x(self, qubit, theta):
        self.circuit.rx(theta, qubit)

    def y(self, qubit, theta):
        self.circuit.ry(theta, qubit)

    def z(self, qubit, theta):
        self.circuit.rz(theta, qubit)

    def swap(self, qubit1, qubit2):
        self.circuit.swap(qubit1, qubit2)
        return self

    def reset(self, qubit):
        self.circuit.reset(qubit)

    def measure(self, qubit):
        self.circuit.measure(qubit, self.bit_count)
        self.bit_count = self.bit_count + 1
        return self

    def clear(self):
        self.circuit.clear()
        self.bit_count = 0


class QASMRuntime(RuntimeAdaptor):
    def execute(self, builder: QASMBuilder) -> Dict[str, int]:
        aer_config = QasmBackendConfiguration.from_dict(AerSimulator._DEFAULT_CONFIGURATION)
        aer_config.n_qubits = builder.circuit.num_qubits
        qasm_sim = AerSimulator(aer_config)

        circuit = builder.circuit
        # TODO: Needs a more nuanced try/catch. Some exceptions we should catch, others we should re-throw.
        try:
            job = qasm_sim.run(transpile(circuit, qasm_sim), shots=builder.shot_count)
            results = job.result()
            distribution = results.get_counts() # Used to pass in circuit, check.
        except QiskitError as e:
            raise ValueError(f"Error while attempting to build/run circuit: {str(e)}")

        removals = builder.circuit.num_qubits - builder.bit_count

        # Because qiskit needs all values up-front we just provide a maximal classical register then strim off
        # the values we aren't going to use.
        return {key[removals:]: value for key, value in distribution.items()}
