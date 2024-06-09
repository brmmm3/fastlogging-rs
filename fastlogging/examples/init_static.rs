use std::io::Error;

use fastlogging::logging_init;

fn main() -> Result<(), Error> {
    let logger = logging_init();
    logger.trace("Trace Message")?;
    logger.debug("Debug Message")?;
    logger.info("Info Message")?;
    logger.success("Success Message")?;
    logger.error("Error Message")?;
    logger.fatal("Fatal Message")?;
    logger.sync_all(1.0)?;
    Ok(())
}
