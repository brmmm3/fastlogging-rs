use fastlogging::{Logging, LoggingError};

fn main() -> Result<(), LoggingError> {
    let mut logger = Logging::default();
    logger.info("Info Message")?;
    logger.debug("Debug Message")?;
    logger.error("Error Message")?;
    logger.shutdown(false)?;
    Ok(())
}
