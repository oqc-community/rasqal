from typing import Dict


def build_args():
    return [1, "Dave", 2.5]


def build_invalid_args():
    return [None, object(), BuilderAdaptor()]


class BuilderAdaptor:
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

    def measure(self, qubit, register):
        self.gates.append(f"measure {qubit} {register}")

    def clear(self):
        self.gates.append("clear")


class RuntimeAdaptor:
    def execute(self, _: BuilderAdaptor) -> Dict[str, int]:
        return {"00": 250, "01": 250, "10": 250, "11": 251}
