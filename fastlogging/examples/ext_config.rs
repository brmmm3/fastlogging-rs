use fastlogging::{
    ConsoleWriterConfig, DEBUG, ExtConfig, Logging, LoggingError, MessageStructEnum,
};

fn main() -> Result<(), LoggingError> {
    let mut logger = Logging::new(
        DEBUG,
        "root",
        Some(vec![ConsoleWriterConfig::new(DEBUG, true).into()]),
        None,
        None,
    )?;
    let ext_config = ExtConfig::new(MessageStructEnum::Xml, true, false, true, false, true);
    logger.set_ext_config(&ext_config);
    println!("ext_config.structured={:?}", ext_config.structured);
    println!("ext_config.hostname={}", ext_config.hostname);
    println!("ext_config.pname={}", ext_config.pname);
    println!("ext_config.pid={}", ext_config.pid);
    println!("ext_config.tname={}", ext_config.tname);
    println!("ext_config.tid={}", ext_config.tid);
    logger.trace("Trace Message")?;
    logger.debug("Debug Message")?;
    logger.info("Info Message")?;
    logger.success("Success Message")?;
    logger.error("Error Message")?;
    logger.fatal("Fatal Message")?;
    logger.shutdown(false)?;
    Ok(())
}
