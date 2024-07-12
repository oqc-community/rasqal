# SPDX-License-Identifier: BSD-3-Clause
# Copyright (c) 2024 Oxford Quantum Circuits Ltd

from typing import Dict


class BuilderAdaptor:
    """
    Python APIs which Rasqals internal features demand exist.

    This builder will be called when it has a quantum blob it wants to execute.
    Each method will be called when that gate/instruction needs to be executed, with the assumption that it will
    transform it into a form which the backend can then execute.

    This builder will then be passed to the provided runtime for execution.
    """

    def cx(self, controls, target, radii): ...

    def cz(self, controls, target, radii): ...

    def cy(self, controls, target, radii): ...

    def x(self, qubit, radii): ...

    def y(self, qubit, radii): ...

    def z(self, qubit, radii): ...

    def swap(self, qubit1, qubit2): ...

    def reset(self, qubit): ...

    def measure(self, qubit): ...


class RuntimeAdaptor:
    """
    Python API which Rasqal expects to be in place and holds central calls for extracting feature
    capabilities and running built-up builders.

    It can model a single QPU/simulator or a collection of them.

    Every time a quantum blob needs to be executed it will query whether a particular runtime is able to support
    it and then use that builder/runtime combination to execute it, if applicable.
    """

    def execute(self, builder) -> Dict[str, int]:
        """
        Executes the passed-in builder against the backend and returns a result distribution.

        The builder can be expected to be the same as returned from the associated `create_builder` function.
        """
        return dict()

    def create_builder(self) -> BuilderAdaptor:
        """Creates a builder to be used with this runtime."""
        return BuilderAdaptor()

    def has_features(self, required_features: "RequiredFeatures"):
        """
        Checks whether this QPU has the required features to execute the builder.
        """
        return True


class RequiredFeatures:
    qubit_count: int
