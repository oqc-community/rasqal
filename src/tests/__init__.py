import os
import pathlib

from rasqal.utils import initialize_logger, DEFAULT_LOG_FOLDER

# Automatically initialize file logging for tests.
initialize_logger(
    os.path.join(
        pathlib.Path(__file__).parent.parent.parent.resolve(),
        DEFAULT_LOG_FOLDER,
        "tests_log.txt",
    )
)
