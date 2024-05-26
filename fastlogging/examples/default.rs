use std::io::Error;

use fastlogging::Logging;

fn main() -> Result<(), Error> {
    let mut logger = Logging::default();
    logger.info("Hello1".to_string())?;
    logger.debug("Hello2".to_string())?;
    logger.error("Hello3".to_string())?;
    logger.shutdown(false)?;
    Ok(())
}
