use std::io::Error;

use fastlogging::DEFAULT_LOGGER;

fn main() -> Result<(), Error> {
    let logger = DEFAULT_LOGGER.lock().unwrap();
    logger.trace("Trace Message")?;
    logger.debug("Debug Message")?;
    logger.info("Info Message")?;
    logger.success("Success Message")?;
    logger.error("Error Message")?;
    logger.fatal("Fatal Message")?;
    logger.sync_all(1.0)?;
    Ok(())
}
