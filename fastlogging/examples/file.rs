use std::path::PathBuf;

use fastlogging::{CompressionMethodEnum, DEBUG, FileWriterConfig, Logging, LoggingError};

fn main() -> Result<(), LoggingError> {
    let mut logger = Logging::new_unboxed(
        DEBUG,
        "root",
        Some(vec![
            FileWriterConfig::new(
                DEBUG,
                PathBuf::from("/tmp/cfastlogging.log"),
                1024,
                3,
                None,
                None,
                Some(CompressionMethodEnum::Store),
            )?
            .into(),
        ]),
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
