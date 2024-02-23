# SPDX-License-Identifier: BSD-3-Clause
# Copyright (c) 2024 Oxford Quantum Circuits Ltd

from os import remove
from os.path import dirname, exists, join
from tempfile import NamedTemporaryFile
from typing import Any, List

from pykin import initialize_logger, BuilderAdaptor, RuntimeAdaptor, Executor, DEFAULT_LOG_FILE

dev_directory = join(dirname(__file__), "..", "..", "..", "munchkin")

# Enable file logging if we're in a development environment.
if exists(dev_directory):
    initialize_logger(join(f"{dev_directory}", f"{DEFAULT_LOG_FILE}"))


class MunchkinRuntime:
    """
    Wrapper API for native functionality, purely in Python to help do mappings.
    """

    def __init__(self, builder: BuilderAdaptor, runtime: RuntimeAdaptor):
        self.builder: BuilderAdaptor = builder
        self.runtime: RuntimeAdaptor = runtime
        self.executor = Executor()

    def trace_graphs(self):
        self.executor.trace_graphs()
        return self

    def trace_projections(self):
        self.executor.trace_projections()
        return self

    def trace_runtime(self):
        self.executor.trace_runtime()
        return self

    def run_ll(self, ll_string: str, args: List[Any] = None):
        """Runs a .ll string. Creates temporary file and writes to it."""
        with NamedTemporaryFile(suffix=".ll", delete=False) as fp:
            fp.write(ll_string)
            fp.close()
            try:
                return self.run(fp.name, args)
            finally:
                remove(fp.name)

    def run_bitcode(self, bitcode: bytes, args: List[Any] = None):
        """Runs LLVM bitcode when passed as bytes. Creates temporary file and writes to it."""
        with NamedTemporaryFile(suffix=".bc", delete=False) as fp:
            fp.write(bitcode)
            fp.close()
            try:
                return self.run(fp.name, args)
            finally:
                remove(fp.name)

    def run(self, file_path: str, args: List[Any] = None):
        results = self.executor.run_with_args(
            file_path, args or [], self.builder, self.runtime
        )
        return results
