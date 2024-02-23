# SPDX-License-Identifier: BSD-3-Clause
# Copyright (c) 2024 Oxford Quantum Circuits Ltd

from typing import Dict


class BuilderAdaptor:
    def cx(self, controls, target, radii):
        ...

    def cz(self, controls, target, radii):
        ...

    def cy(self, controls, target, radii):
        ...

    def x(self, qubit, radii):
        ...

    def y(self, qubit, radii):
        ...

    def z(self, qubit, radii):
        ...

    def swap(self, qubit1, qubit2):
        ...

    def reset(self, qubit):
        ...

    def measure(self, qubit):
        ...

    def clear(self):
        ...


class RuntimeAdaptor:
    def execute(self, builder) -> Dict[str, int]:
        ...
