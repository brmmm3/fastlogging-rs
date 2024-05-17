use std::io::Error;

use fastlogging::Logging;

fn main() -> Result<(), Error> {
    let mut logger = Logging::new(None, None, None, None, None, None, None, None)?;
    logger.info("Hello1".to_string())?;
    logger.debug("Hello2".to_string())?;
    logger.error("Hello3".to_string())?;
    logger.shutdown(None)?;
    Ok(())
}
