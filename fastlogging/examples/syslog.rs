use std::io::Error;

use fastlogging::{Logging, DEBUG};

fn main() -> Result<(), Error> {
    let mut logger = Logging::new(None, None, None, None, None, None, None, Some(DEBUG), None)?;
    logger.trace("Trace Message")?;
    logger.debug("Debug Message")?;
    logger.info("Info Message")?;
    logger.success("Success Message")?;
    logger.error("Error Message")?;
    logger.fatal("Fatal Message")?;
    logger.shutdown(false)?;
    Ok(())
}
