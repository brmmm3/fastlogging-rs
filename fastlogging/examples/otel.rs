use fastlogging::{
    DEBUG, DEFAULT_OTEL_ENDPOINT, DEFAULT_OTEL_SERVICE_NAME, Logging, LoggingError,
    OpenTelemetryWriterConfig,
};

fn main() -> Result<(), LoggingError> {
    // Create a new logging instance with an OpenTelemetry writer
    // pointing to a local OpenTelemetry Collector.
    let mut logger = Logging::new(
        DEBUG,
        "root",
        Some(vec![
            OpenTelemetryWriterConfig::new(DEBUG, DEFAULT_OTEL_ENDPOINT, DEFAULT_OTEL_SERVICE_NAME)
                .into(),
        ]),
        None,
        None,
    )?;

    // Log messages at various levels — they will be exported to the OTel Collector
    logger.trace("Trace Message")?;
    logger.debug("Debug Message")?;
    logger.info("Info Message")?;
    logger.success("Success Message")?;
    logger.warning("Warning Message")?;
    logger.error("Error Message")?;
    logger.fatal("Fatal Message")?;

    // Shutdown flushes any remaining buffered log records
    logger.shutdown(false)?;
    Ok(())
}
