from pyfastlogging import (
    TRACE,
    Logging,
    OpenTelemetryWriterConfig,
)

if __name__ == "__main__":
    # Create a logging instance with an OpenTelemetry writer
    # pointing to a local OpenTelemetry Collector.
    logger = Logging(
        TRACE,
        "main",
        otel=OpenTelemetryWriterConfig(
            level=TRACE,
            endpoint="http://localhost:4318",
            service_name="my-python-app",
        ),
    )
    logger.trace("Trace Message")
    logger.debug("Debug Message")
    logger.info("Info Message")
    logger.success("Success Message")
    logger.warning("Warning Message")
    logger.error("Error Message")
    logger.fatal("Fatal Message")
    logger.shutdown()
