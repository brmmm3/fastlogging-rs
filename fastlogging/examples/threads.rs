use std::thread;

use fastlogging::{
    ConsoleWriterConfig, DEBUG, ExtConfig, Logger, Logging, LoggingError, MessageStructEnum,
};

fn main() -> Result<(), LoggingError> {
    let mut logging = Logging::default();
    logging.set_ext_config(&mut ExtConfig::new(
        MessageStructEnum::String,
        true,
        true,
        true,
        true,
        true,
    ));
    logging.add_writer_config(&ConsoleWriterConfig::new(DEBUG, true).into())?;
    let logger = Logger::new_ext(DEBUG, "LoggerThread", true, true);
    logging.add_logger(&mut logger);
    let thr = thread::Builder::new()
        .name("SomeThread".to_string())
        .spawn(move || {
            logger
                .trace("Trace Message")
                .expect("Failed to log message");
            logger
                .debug("Debug Message")
                .expect("Failed to log message");
            logger.info("Info Message").expect("Failed to log message");
            logger
                .success("Success Message")
                .expect("Failed to log message");
            logger
                .error("Error Message")
                .expect("Failed to log message");
            logger
                .fatal("Fatal Message")
                .expect("Failed to log message");
            logger.flush(0.0);
        })?;
    logging.trace("Trace Message")?;
    logging.debug("Debug Message")?;
    logging.info("Info Message")?;
    logging.success("Success Message")?;
    logging.error("Error Message")?;
    logging.fatal("Fatal Message")?;
    thr.join().unwrap();
    println!("JOINED");
    logging.shutdown(false)?;
    Ok(())
}
