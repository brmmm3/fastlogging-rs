import fastlogging_rs as fl
from fastlogging_rs import (
    TRACE,
    Logging,
    ConsoleWriterConfig,
)

if __name__ == "__main__":
    fl.trace("Trace Message")
    fl.debug("Debug Message")
    fl.info("Info Message")
    fl.success("Success Message")
    fl.warning("Warning Message")
    fl.error("Error Message")
    fl.fatal("Fatal Message")
