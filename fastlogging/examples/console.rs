use std::io::Error;

use fastlogging::{ConsoleWriterConfig, Logging, DEBUG};

fn main() -> Result<(), Error> {
    let console = ConsoleWriterConfig::new(DEBUG, true);
    let mut logger = Logging::new(
        None,
        None,
        None,
        Some(console),
        None,
        None,
        None,
        None,
        None,
    )?;
    logger.trace("Trace Message")?;
    logger.debug("Debug Message")?;
    logger.info("Info Message")?;
    logger.success("Success Message")?;
    logger.error("Error Message")?;
    logger.fatal("Fatal Message")?;
    logger.shutdown(false)?;
    Ok(())
}
