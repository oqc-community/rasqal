# SPDX-License-Identifier: BSD-3-Clause
# Copyright (c) 2024 Oxford Quantum Circuits Ltd

from typing import Optional

from ._native import initialize_file_logger, initialize_commandline_logger


def initialize_logger(file_path: Optional[str] = None):
    if file_path is None:
        initialize_commandline_logger()
    else:
        initialize_file_logger(file_path)
