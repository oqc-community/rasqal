# SPDX-License-Identifier: BSD-3-Clause
# Copyright (c) 2024 Oxford Quantum Circuits Ltd

from os import remove
from os.path import dirname, exists, join
from tempfile import NamedTemporaryFile
from typing import Any, List, Union

from .utils import initialize_logger
from .adaptors import RuntimeAdaptor
from ._native import DEFAULT_LOG_FILE, Executor

dev_directory = join(dirname(__file__), "..", "..", "..", "rasqalkin")

# Enable file logging if we're in a development environment.
if exists(dev_directory):
    initialize_logger(join(f"{dev_directory}", f"{DEFAULT_LOG_FILE}"))


class RasqalRunner:
    """
    Provides a wrapper around the Rust implementation details, allowing more natural extension
    from Python as well as utility and supporting methods.
    """

    def __init__(self, runtime: Union[List[RuntimeAdaptor], RuntimeAdaptor]):
        if not isinstance(runtime, list):
            runtime = [runtime]

        self.runtimes: List[RuntimeAdaptor] = runtime
        self.executor = Executor()

    def trace_graphs(self) -> "RasqalRunner":
        """
        Activates graph logging.
        Prints out the active execution graphs before running.
        """
        self.executor.trace_graphs()
        return self

    def trace_projections(self) -> "RasqalRunner":
        """
        Activates projection logging.
        Holds information in regards to value prediction as well as what circuit is actually built.
        """
        self.executor.trace_projections()
        return self

    def trace_runtime(self) -> "RasqalRunner":
        """
        Activates runtime logging.
        Prints every step the symbolic executor takes.
        """
        self.executor.trace_runtime()
        return self

    def run_ll(self, ll_string: str, args: List[Any] = None):
        """Runs a .ll string. Creates temporary file and writes to it."""
        # Need to set as string not bytes for encoding purposes.
        with NamedTemporaryFile(suffix=".ll", delete=False, mode="w+") as fp:
            fp.write(ll_string)
            fp.close()
            try:
                return self.run(fp.name, args)
            finally:
                remove(fp.name)

    def step_count_limit(self, step_count: int) -> "RasqalRunner":
        """
        Sets a limit for the steps the symbolic executor can take.

        If the environment this is running in is resource-constrained or heavily utilized you can set
        a limit on how many steps it can take. It'll mean more complicated code will error out, but will bound
        how long a run can take.

        Smaller number == shorter path it's allowed to walk. Does not take into account time it takes for backend to
        process and run quantum circuits though.
        """
        self.executor.step_count_limit(step_count)
        return self

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
        """
        Runs an .ll or .bc file with the passed-in arguments.
        Arguments can only be Python primitives or otherwise easily transformable to Rust objects.
        """
        results = self.executor.run_with_args(
            file_path, args or [], self.runtimes
        )
        return results
