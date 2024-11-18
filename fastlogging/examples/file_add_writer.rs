use std::path::PathBuf;

use fastlogging::{CompressionMethodEnum, FileWriterConfig, Logging, LoggingError, DEBUG};

fn main() -> Result<(), LoggingError> {
    let mut logger = Logging::new(DEBUG, "root", Vec::new(), None, None)?;
    logger.add_writer_config(
        &FileWriterConfig::new(
            DEBUG,
            PathBuf::from("/tmp/cfastlogging.log"),
            1024,
            3,
            None,
            None,
            Some(CompressionMethodEnum::Store),
        )?
        .into(),
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
