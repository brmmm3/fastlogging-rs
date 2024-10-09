from fastlogging_rs import (
    TRACE,
    DEBUG,
    Logging,
    ConsoleWriterConfig,
    CallbackWriterConfig,
)


def writer_callback(level: int, domain: str, message: str):
    print(f"--> {level} {domain} {message}")


if __name__ == "__main__":
    logger = Logging(
        TRACE,
        "main",
        [
            ConsoleWriterConfig(TRACE, True),
            CallbackWriterConfig(DEBUG, writer_callback),
        ],
    )
    logger.trace("Trace Message")
    logger.debug("Debug Message")
    logger.info("Info Message")
    logger.success("Success Message")
    logger.warning("Warning Message")
    logger.error("Error Message")
    logger.fatal("Fatal Message")
    logger.shutdown()
