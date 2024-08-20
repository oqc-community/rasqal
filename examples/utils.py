from copy import copy

from rasqal.adaptors import BuilderAdaptor, RuntimeAdaptor


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
        self.gates.clear()


class RuntimeMock(RuntimeAdaptor):
    def __init__(self, suppress_printout=False):
        self.executed = []
        self.results = {
            "00": 100
        }
        self.suppress = suppress_printout

    def execute(self, builder: BuilderMock):
        # Store our gates if we need to look at them.
        self.executed.append(copy(builder.gates))

        if not self.suppress:
            print("Running circuit:")
            for gate in builder.gates:
                print(gate)

        # Then return the results we're mocking.
        return self.results

    def create_builder(self) -> BuilderAdaptor:
        # Returning our builder mock every time.
        return BuilderMock()

    def has_features(self, required_features):
        # We just state that we can run anything
        return True
