import os
import pathlib

from rasqal.utils import initialize_logger

# Automatically initialize file logging for tests.
initialize_logger(os.path.join(pathlib.Path(__file__).parent.resolve(), "rasqal.txt"))
