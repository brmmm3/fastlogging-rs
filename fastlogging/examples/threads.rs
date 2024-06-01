use std::{io::Error, thread};

use fastlogging::{
    ConsoleWriterConfig, ExtConfig, Logger, Logging, MessageStructEnum, WriterConfigEnum,
    WriterTypeEnum, DEBUG,
};

fn main() -> Result<(), Error> {
    let mut logger = Logging::default();
    logger.set_ext_config(ExtConfig::new(
        MessageStructEnum::String,
        true,
        true,
        true,
        true,
        true,
    ));
    logger.add_writer(WriterConfigEnum::Console(ConsoleWriterConfig::new(
        DEBUG, true,
    )))?;
    let mut logger2 = Logger::new_ext(DEBUG, "Thread1", true, true);
    logger.add_logger(&mut logger2);
    let thr = thread::Builder::new()
        .name("SomeThread".to_string())
        .spawn(move || {
            logger2.info("Info Message").expect("Failed to log message");
            logger2
                .debug("Debug Message")
                .expect("Failed to log message");
            logger2
                .error("Error Message")
                .expect("Failed to log message");
        })?;
    logger.info("Info Message")?;
    logger.debug("Debug Message")?;
    logger.error("Error Message")?;
    thr.join().unwrap();
    logger.shutdown(false)?;
    Ok(())
}
