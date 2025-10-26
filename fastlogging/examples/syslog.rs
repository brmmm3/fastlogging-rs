use fastlogging::{DEBUG, Logging, LoggingError};

fn main() -> Result<(), LoggingError> {
    let mut logger = Logging::new(DEBUG, "root", None, None, None)?;
    logger.trace("Trace Message")?;
    logger.debug("Debug Message")?;
    logger.info("Info Message")?;
    logger.success("Success Message")?;
    logger.error("Error Message")?;
    logger.fatal("Fatal Message")?;
    logger.shutdown(false)?;
    Ok(())
}
