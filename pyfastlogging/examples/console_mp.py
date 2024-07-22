import os

import fastlogging_rs
from fastlogging_rs import (
    TRACE,
    Logging,
    ConsoleWriterConfig,
)

print("__name__", __name__, os.getpid())


def ChildProcess():
    print("ChildProcess", __name__, os.getpid())
    logger = Logging(
        TRACE,
        "child",
        console=ConsoleWriterConfig(TRACE, True),
    )
    logger.trace("Trace Message")
    logger.debug("Debug Message")
    logger.info("Info Message")
    logger.success("Success Message")
    logger.warning("Warning Message")
    logger.error("Error Message")
    logger.fatal("Fatal Message")
    logger.shutdown()


if __name__ == "__main__":
    import multiprocessing
    from multiprocessing import Pool, freeze_support

    multiprocessing.set_start_method("spawn")
    freeze_support()
    with Pool() as pool:
        pool.apply(ChildProcess)
        pool.apply(ChildProcess)
    logger = Logging(
        TRACE,
        "main",
        console=ConsoleWriterConfig(TRACE, True),
    )
    logger.trace("Trace Message")
    logger.debug("Debug Message")
    logger.info("Info Message")
    logger.success("Success Message")
    logger.warning("Warning Message")
    logger.error("Error Message")
    logger.fatal("Fatal Message")
    logger.shutdown()
