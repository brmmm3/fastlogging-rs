use fastlogging::{Logging, LoggingError};

fn main() -> Result<(), LoggingError> {
    let mut logger = Logging::default();
    logger.trace("Trace Message")?;
    logger.debug("Debug Message")?;
    logger.info("Info Message")?;
    logger.success("Success Message")?;
    logger.error("Error Message")?;
    logger.fatal("Fatal Message")?;
    logger.shutdown(false)?;
    Ok(())
}
