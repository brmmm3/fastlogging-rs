import os

import pyfastlogging as fl
from pyfastlogging import (
    TRACE,
    Logging,
    CompressionMethodEnum,
    FileWriterConfig,
)

if __name__ == "__main__":
    pathName = (
        "C:\\temp\\pyfastlogging.log" if os.name == "nt" else "/tmp/pyfastlogging.log"
    )
    logger = Logging(TRACE, "main")
    logger.add_writer(FileWriterConfig(TRACE, pathName, compression=CompressionMethodEnum.Deflate))
    fl.trace("Trace Message")
    fl.debug("Debug Message")
    fl.info("Info Message")
    fl.success("Success Message")
    fl.warning("Warning Message")
    fl.error("Error Message")
    fl.fatal("Fatal Message")
