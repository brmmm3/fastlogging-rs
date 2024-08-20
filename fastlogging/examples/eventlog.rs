// Important: Must be run with admin rights!

use fastlogging::{Logging, LoggingError, DEBUG};

fn main() -> Result<(), LoggingError> {
    eventlog::register("fastlogging").map_err(|e| LoggingError::InvalidValue(e.to_string()))?;
    let mut logger = Logging::new(None, None, None, None, None, None, None, Some(DEBUG), None)?;
    logger.trace("Trace Message")?;
    logger.debug("Debug Message")?;
    logger.info("Info Message")?;
    logger.success("Success Message")?;
    logger.error("Error Message")?;
    logger.fatal("Fatal Message")?;
    logger.shutdown(false)?;
    eventlog::deregister("fastlogging").map_err(|e| LoggingError::InvalidValue(e.to_string()))?;
    Ok(())
}
