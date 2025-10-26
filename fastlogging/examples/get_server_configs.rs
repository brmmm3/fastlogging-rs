use std::{thread, time::Duration};

use fastlogging::{
    ConsoleWriterConfig, DEBUG, EncryptionMethod, Logging, LoggingError, ServerConfig,
};

fn main() -> Result<(), LoggingError> {
    // Server
    let mut logging_server = Logging::new_unboxed(
        DEBUG,
        "LOGSRV",
        Some(vec![
            ConsoleWriterConfig::new(DEBUG, true).into(),
            ServerConfig::new(DEBUG, "127.0.0.1", EncryptionMethod::NONE).into(),
        ]),
        None,
        None,
    )?;
    // Set root writer
    logging_server.set_root_writer_config(
        &ServerConfig::new(DEBUG, "127.0.0.1", EncryptionMethod::NONE).into(),
    )?;
    //logging_server.set_debug(3);
    logging_server.sync_all(5.0).unwrap();
    // Show configs
    let configs = logging_server.get_server_configs();
    println!("configs={configs:#?}");
    // Remove ROOT writer
    let writers = logging_server.remove_writers(Some(vec![0]));
    println!("REMOVED={writers:#?}");
    // Show configs
    let configs2 = logging_server.get_server_configs();
    println!("configs2={configs2:#?}");
    // Test logging
    println!("Send logs");
    logging_server.trace("Trace Message".to_string()).unwrap();
    logging_server.debug("Debug Message".to_string()).unwrap();
    logging_server.info("Info Message".to_string()).unwrap();
    logging_server
        .success("Success Message".to_string())
        .unwrap();
    logging_server
        .warning("Warning Message".to_string())
        .unwrap();
    logging_server.error("Error Message".to_string()).unwrap();
    logging_server.fatal("Fatal Message".to_string()).unwrap();

    logging_server.sync_all(1.0)?;
    // Give client some time to send the log messages
    thread::sleep(Duration::from_millis(50));
    println!("Shutdown Loggers");
    logging_server.shutdown(false).unwrap();
    println!("-------- Finished --------");
    Ok(())
}
