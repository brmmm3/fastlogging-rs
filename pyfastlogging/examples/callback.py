from fastlogging_rs import TRACE, Logging, ConsoleWriterConfig, CallbackWriterConfig


def writer_callback(level: int, domain: str, message: str):
    print(f"#writer_callback# {level} {domain} {message}")


if __name__ == "__main__":
    logger = Logging(
        TRACE,
        "main",
        console=ConsoleWriterConfig(TRACE, True),
    )
    callback_writer = CallbackWriterConfig(DEBUG, writer_callback)
    logger.add_writer(callback_writer)
    logger.trace("Trace Message")
    logger.debug("Debug Message")
    logger.info("Info Message")
    logger.success("Success Message")
    logger.warning("Warning Message")
    logger.error("Error Message")
    logger.fatal("Fatal Message")
    logger.shutdown()
