# SPDX-License-Identifier: BSD-3-Clause
# Copyright (c) 2024 Oxford Quantum Circuits Ltd

from typing import Optional

from ._native import (
    initialize_file_logger,
    initialize_commandline_logger,
    DEFAULT_LOG_FILE,  # noqa
    DEFAULT_LOG_FOLDER,  # noqa
)


def initialize_logger(file_path: Optional[str] = None):
    """
    Initializes the Rust logger from Python. You'll have to call this with a valid path before any calls to the native
    code otherwise logging will have already been initialized and it'll be ignored.
    """
    if file_path is None:
        initialize_commandline_logger()
    else:
        initialize_file_logger(file_path)
