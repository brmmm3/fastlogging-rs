import tempfile
from threading import Thread

from pyfastlogging import (
    TRACE,
    DEBUG,
    MessageStructEnum,
    Logging,
    Logger,
    ExtConfig,
    ConsoleWriterConfig,
    FileWriterConfig,
)


def SomeThread(logger):
    logger.trace("Trace Message")
    logger.debug("Debug Message")
    logger.info("Info Message")
    logger.success("Success Message")
    logger.warning("Warning Message")
    logger.error("Error Message")
    logger.fatal("Fatal Message")


if __name__ == "__main__":
    tmpDir = tempfile.mkdtemp(prefix="fastlogging")
    logger = Logging(TRACE)
    logger.set_ext_config(
        ExtConfig(MessageStructEnum.String, True, True, True, True, True)
    )
    logger.add_writer(ConsoleWriterConfig(TRACE, True))
    logger.add_writer(FileWriterConfig(TRACE, f"{tmpDir}/fastlogging.log"))
    logger2 = Logger(DEBUG, "LoggerThread", None, True, True)
    logger.add_logger(logger2)
    thr = Thread(target=SomeThread, args=(logger2,), daemon=True)
    thr.start()
    logger.trace("Trace Message")
    logger.debug("Debug Message")
    logger.info("Info Message")
    logger.success("Success Message")
    logger.warning("Warning Message")
    logger.error("Error Message")
    logger.fatal("Fatal Message")
    thr.join()
    logger.shutdown()
