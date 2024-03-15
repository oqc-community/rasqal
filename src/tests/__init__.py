import os
import pathlib

from rasqal.utils import initialize_logger, DEFAULT_LOG_FILE

# Automatically initialize file logging for tests.
initialize_logger(os.path.join(pathlib.Path(__file__).parent.resolve(), DEFAULT_LOG_FILE))
