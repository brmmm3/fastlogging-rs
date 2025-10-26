use fastlogging::{CallbackWriterConfig, DEBUG, Logging, LoggingError};

fn writer_callback(level: u8, domain: String, message: String) -> Result<(), LoggingError> {
    println!("CB: {level} {domain}: {message}");
    Ok(())
}

fn main() -> Result<(), LoggingError> {
    let mut logging = Logging::new_unboxed(
        DEBUG,
        "root",
        Some(vec![
            CallbackWriterConfig::new(DEBUG, Some(Box::new(writer_callback))).into(),
        ]),
        None,
        None,
    )
    .unwrap();
    logging.trace("Trace Message".to_string()).unwrap();
    logging.debug("Debug Message".to_string()).unwrap();
    logging.info("Info Message".to_string()).unwrap();
    logging.success("Success Message".to_string()).unwrap();
    logging.warning("Warning Message".to_string()).unwrap();
    logging.error("Error Message".to_string()).unwrap();
    logging.fatal("Fatal Message".to_string()).unwrap();
    logging.shutdown(false).unwrap();
    Ok(())
}
