from copy import copy

from rasqal.adaptors import BuilderAdaptor
from rasqal.simulators import QASMBuilder, QASMRuntime


class TracingBuilder(QASMBuilder):
    def __init__(self, qubit_count: int):
        super().__init__(qubit_count)
        self.gates = []

    def cx(self, controls, target, radii):
        self.gates.append(f"cx {controls} {target} {radii}")
        super().cx(controls, target, radii)

    def cz(self, controls, target, radii):
        self.gates.append(f"cz {controls} {target} {radii}")
        super().cz(controls, target, radii)

    def cy(self, controls, target, radii):
        self.gates.append(f"cy {controls} {target} {radii}")
        super().cy(controls, target, radii)

    def x(self, qubit, radii):
        self.gates.append(f"x {qubit} {radii}")
        super().x(qubit, radii)

    def y(self, qubit, radii):
        self.gates.append(f"y {qubit} {radii}")
        super().y(qubit, radii)

    def z(self, qubit, radii):
        self.gates.append(f"z {qubit} {radii}")
        super().z(qubit, radii)

    def swap(self, qubit1, qubit2):
        self.gates.append(f"swap {qubit1} {qubit2}")
        super().swap(qubit1, qubit2)

    def reset(self, qubit):
        self.gates.append(f"reset {qubit}")
        super().reset(qubit)

    def measure(self, qubit):
        self.gates.append(f"measure {qubit}")
        super().measure(qubit)


class TracingRuntime(QASMRuntime):
    def __init__(self, qubit_count=None):
        self.executed = []
        if qubit_count is not None:
            super().__init__(qubit_count)
        else:
            super().__init__()

    def execute(self, builder: TracingBuilder):
        # Store our gates if we need to look at them.
        self.executed.append(copy(builder.gates))

        print("Running circuit:")
        for gate in builder.gates:
            print(gate)

        # Then return the results we're mocking.
        return super().execute(builder)

    def create_builder(self) -> BuilderAdaptor:
        # Returning our builder mock every time.
        return TracingBuilder(self.qubit_count)

    def has_features(self, required_features):
        # We just state that we can run anything
        return True
