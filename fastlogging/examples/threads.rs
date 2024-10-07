use std::thread;

use fastlogging::{
    ConsoleWriterConfig, ExtConfig, Logger, Logging, LoggingError, MessageStructEnum, DEBUG,
};

fn main() -> Result<(), LoggingError> {
    let mut logger = Logging::default();
    logger.set_ext_config(&mut ExtConfig::new(
        MessageStructEnum::String,
        true,
        true,
        true,
        true,
        true,
    ));
    logger.add_writer_config(&ConsoleWriterConfig::new(DEBUG, true).into())?;
    let mut logger2 = Logger::new_ext(DEBUG, "LoggerThread", true, true);
    logger.add_logger(&mut logger2);
    let thr = thread::Builder::new()
        .name("SomeThread".to_string())
        .spawn(move || {
            logger2
                .trace("Trace Message")
                .expect("Failed to log message");
            logger2
                .debug("Debug Message")
                .expect("Failed to log message");
            logger2.info("Info Message").expect("Failed to log message");
            logger2
                .success("Success Message")
                .expect("Failed to log message");
            logger2
                .error("Error Message")
                .expect("Failed to log message");
            logger2
                .fatal("Fatal Message")
                .expect("Failed to log message");
        })?;
    logger.trace("Trace Message")?;
    logger.debug("Debug Message")?;
    logger.info("Info Message")?;
    logger.success("Success Message")?;
    logger.error("Error Message")?;
    logger.fatal("Fatal Message")?;
    thr.join().unwrap();
    logger.shutdown(false)?;
    Ok(())
}
