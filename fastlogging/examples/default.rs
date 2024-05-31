use std::io::Error;

use fastlogging::Logging;

fn main() -> Result<(), Error> {
    let mut logger = Logging::default();
    logger.info("Info Message")?;
    logger.debug("Debug Message")?;
    logger.error("Error Message")?;
    logger.shutdown(false)?;
    Ok(())
}
