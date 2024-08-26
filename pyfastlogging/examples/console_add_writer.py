from fastlogging_rs import (
    TRACE,
    Logging,
    ConsoleWriterConfig,
)

if __name__ == "__main__":
    logger = Logging(TRACE, "main")
    logger.add_writer(ConsoleWriterConfig(TRACE, True))
    logger.trace("Trace Message")
    logger.debug("Debug Message")
    logger.info("Info Message")
    logger.success("Success Message")
    logger.warning("Warning Message")
    logger.error("Error Message")
    logger.fatal("Fatal Message")
    logger.shutdown()
