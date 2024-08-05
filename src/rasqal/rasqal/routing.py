from typing import Dict, Tuple, List, Union

from .runtime import RasqalRunner
from pytket.architecture import Architecture
from pytket import Circuit, OpType, Qubit, Bit
from pytket.passes import SequencePass, DefaultMappingPass

from .adaptors import BuilderAdaptor, RuntimeAdaptor


def build_ring_architecture(num_qubits):
    """
    Builds a default ring architecture for the number of qubits supplied. 0->1, 1->2, ...
    """
    return [(i % num_qubits, (i + 1) % num_qubits) for i in range(num_qubits)]


def apply_routing(
    couplings: Union[Architecture, List[Tuple[int, int]]],
    runtime: Union[RasqalRunner, RuntimeAdaptor],
):
    if isinstance(runtime, RasqalRunner):
        runtime.runtimes = [TketRuntime(couplings, rt) for rt in runtime.runtimes]
        return runtime
    elif isinstance(runtime, RuntimeAdaptor):
        return TketRuntime(couplings, runtime)
    else:
        raise ValueError(f"Cannot apply routing to {str(runtime)}")


class TketBuilder(BuilderAdaptor):
    def __init__(self):
        self.circuit = Circuit()

    def cx(self, controls, target, radii):
        self.circuit.add_qubit(Qubit(target), False)
        self.circuit.add_qubit(Qubit(controls[0]), False)
        self.circuit.CRx(radii, controls[0], target)

    def cz(self, controls, target, radii):
        self.circuit.add_qubit(Qubit(target), False)
        self.circuit.add_qubit(Qubit(controls[0]), False)
        self.circuit.CRz(radii, controls[0], target)

    def cy(self, controls, target, radii):
        self.circuit.add_qubit(Qubit(target), False)
        self.circuit.add_qubit(Qubit(controls[0]), False)
        self.circuit.CRy(radii, controls[0], target)

    def x(self, qubit, radii):
        self.circuit.add_qubit(Qubit(qubit), False)
        self.circuit.Rx(radii, qubit)

    def y(self, qubit, radii):
        self.circuit.add_qubit(Qubit(qubit), False)
        self.circuit.Ry(radii, qubit)

    def z(self, qubit, radii):
        self.circuit.add_qubit(Qubit(qubit), False)
        self.circuit.Rz(radii, qubit)

    def swap(self, qubit1, qubit2):
        self.circuit.add_qubit(Qubit(qubit1), False)
        self.circuit.add_qubit(Qubit(qubit2), False)
        self.circuit.SWAP(qubit1, qubit2)

    def reset(self, qubit):
        # We're just using a barrier as a tag for reset for now.
        # Also useful so that things don't move past it.
        self.circuit.add_qubit(Qubit(qubit), False)
        self.circuit.add_barrier([qubit])

    def measure(self, qubit):
        # We don't measure into anything, so just imply qubit index == classical bit index.
        self.circuit.add_qubit(Qubit(qubit), False)
        self.circuit.add_bit(Bit(qubit), False)
        self.circuit.Measure(qubit, qubit)


class TketRuntime(RuntimeAdaptor):
    """
    Uses Tket to apply basic routing to synthesized circuits.
    Can be
    """

    def __init__(
        self,
        couplings: Union[Architecture, List[Tuple[int, int]]],
        forwarded_runtime: RuntimeAdaptor,
    ):
        self.forwarded = forwarded_runtime
        if isinstance(couplings, list):
            self.arch = Architecture(couplings)
        elif isinstance(couplings, Architecture):
            self.arch = couplings
        else:
            raise ValueError(
                f"Invalid architecture or coupling mappings: {str(couplings)}"
            )

    def execute(self, builder) -> Dict[str, int]:
        builder: TketBuilder

        SequencePass([DefaultMappingPass(self.arch)]).apply(builder.circuit)
        return self.forwarded.execute(self._forward_circuit(builder))

    def _forward_circuit(self, builder) -> BuilderAdaptor:
        """Forwards the Tket circuit on to the new builder to be run in the forwarding runtime."""
        fbuilder = self.forwarded.create_builder()
        for gate in builder.circuit:
            if gate.op.type == OpType.Rz:
                fbuilder.z(gate.qubits[0].index[0], gate.op.params[0])
            elif gate.op.type == OpType.Rx:
                fbuilder.x(gate.qubits[0].index[0], gate.op.params[0])
            elif gate.op.type == OpType.Ry:
                fbuilder.y(gate.qubits[0].index[0], gate.op.params[0])
            elif gate.op.type == OpType.CRx:
                fbuilder.cx(
                    [gate.qubits[0].index[0]],
                    gate.qubits[1].index[0],
                    gate.op.params[0],
                )
            elif gate.op.type == OpType.CRy:
                fbuilder.cy(
                    [gate.qubits[0].index[0]],
                    gate.qubits[1].index[0],
                    gate.op.params[0],
                )
            elif gate.op.type == OpType.CRz:
                fbuilder.cz(
                    [gate.qubits[0].index[0]],
                    gate.qubits[1].index[0],
                    gate.op.params[0],
                )
            elif gate.op.type == OpType.SWAP:
                fbuilder.swap(gate.qubits[0].index[0], gate.qubits[1].index[0])
            elif gate.op.type == OpType.Measure:
                fbuilder.measure(gate.qubits[0].index[0])
            elif gate.op.type == OpType.Barrier:
                fbuilder.reset(gate.qubits[0].index[0])

        return fbuilder

    def create_builder(self) -> BuilderAdaptor:
        return TketBuilder()
