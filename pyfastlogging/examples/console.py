from fastlogging_rs import (
    TRACE,
    Logging,
    ConsoleWriterConfig,
)

if __name__ == "__main__":
    logger = Logging(
        TRACE,
        "main",
        console=ConsoleWriterConfig(TRACE, True),
    )
    logger.debug("Debug Message")
    logger.info("Info Message")
    logger.success("Success Message")
    logger.warning("Warning Message")
    logger.error("Error Message")
    logger.fatal("Fatal Message")
    logger.shutdown()
