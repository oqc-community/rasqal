# SPDX-License-Identifier: BSD-3-Clause
# Copyright (c) 2024 Oxford Quantum Circuits Ltd

from typing import Any, Optional, List

from .adaptors import BuilderAdaptor, RuntimeAdaptor


DEFAULT_LOG_FILE = ""

def initialize_file_logger(file_path: str):
    pass

def initialize_commandline_logger():
    pass


class Graph:
    ...


class Executor:
    def trace_graphs(self):
        ...

    def trace_runtime(self):
        ...

    def trace_projections(self):
        ...

    def run(
        self,
        file_path: str,
        builder: BuilderAdaptor,
        runtime: RuntimeAdaptor
    ) -> Any:
        """ Runs this file using the automatically-detected entry-point with no arguments. """

    def run_with_args(
        self,
        file_path: str,
        arguments: List[Any],
        builder: BuilderAdaptor,
        runtime: RuntimeAdaptor
    ) -> Any:
        """ Runs this file using the automatically-detected entry-point. """

    def parse_file(
            self,
            file: str,
            entry_point: Optional[str]
    ) -> Graph:
        """ Evaluates and builds this file into the internal execution graph and returns it. """

    def run_graph(
            self,
            graph: Graph,
            arguments: List[Any],
            builder_adaptor: BuilderAdaptor,
            runtime_adaptor: RuntimeAdaptor
    ) -> Any:
        """ Runs a pre-built execution graph with the passed-in arguments. """