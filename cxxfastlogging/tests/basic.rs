use cxxfastlogging::{Logger, Logging, WriterConfig};

#[test]
fn console_logging() {
    let console = WriterConfig::new_console(fastlogging::DEBUG, true);
    let mut logging = Logging::create(fastlogging::NOTSET, "test", vec![console]).unwrap();
    logging.trace("Trace message").unwrap();
    logging.debug("Debug message").unwrap();
    logging.info("Info message").unwrap();
    logging.success("Success message").unwrap();
    logging.warning("Warning message").unwrap();
    logging.error("Error message").unwrap();
    logging.shutdown(false).unwrap();
}

#[test]
fn default_logging() {
    let mut logging = Logging::new_default().unwrap();
    logging.info("Hello from cxxfastlogging").unwrap();
    logging.shutdown(false).unwrap();
}

#[test]
fn logger_registration() {
    let console = WriterConfig::new_console(fastlogging::DEBUG, false);
    let mut logging = Logging::create(fastlogging::DEBUG, "test", vec![console]).unwrap();
    let mut logger = Logger::create(fastlogging::DEBUG, "sub.domain");
    logging.add_logger(&mut logger);
    logger.info("Message from sub logger").unwrap();
    logging.remove_logger(&mut logger);
    logging.shutdown(false).unwrap();
}
