use fastlogging::{LoggingError, ROOT_LOGGER};

fn main() -> Result<(), LoggingError> {
    let logger = ROOT_LOGGER.lock().unwrap();
    logger.trace("Trace Message")?;
    logger.debug("Debug Message")?;
    logger.info("Info Message")?;
    logger.success("Success Message")?;
    logger.error("Error Message")?;
    logger.fatal("Fatal Message")?;
    logger.sync_all(1.0)?;
    Ok(())
}
