use std::io::Error;

use fastlogging::Logging;

fn main() -> Result<(), Error> {
    let mut logger = Logging::init()?;
    logger.trace("Trace Message")?;
    logger.debug("Debug Message")?;
    logger.info("Info Message")?;
    logger.success("Success Message")?;
    logger.error("Error Message")?;
    logger.fatal("Fatal Message")?;
    logger.shutdown(false)?;
    Ok(())
}
