import os

import fastlogging_rs as fl
from fastlogging_rs import (
    TRACE,
    CompressionMethodEnum,
    FileWriterConfig,
)

if __name__ == "__main__":
    pathName = (
        "C:\\temp\\pyfastlogging.log" if os.name == "nt" else "/tmp/pyfastlogging.log"
    )
    fl.add_writer(
        FileWriterConfig(TRACE, pathName, compression=CompressionMethodEnum.Deflate)
    )
    fl.trace("Trace Message")
    fl.debug("Debug Message")
    fl.info("Info Message")
    fl.success("Success Message")
    fl.warning("Warning Message")
    fl.error("Error Message")
    fl.fatal("Fatal Message")
