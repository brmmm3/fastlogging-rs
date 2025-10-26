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
    // Show addresses and ports
    let ports = logging_server.get_server_ports();
    println!("ports={ports:#?}");
    let addresses = logging_server.get_server_addresses();
    println!("addresses={addresses:#?}");
    let address_ports = logging_server.get_server_addresses_ports();
    println!("address_ports={address_ports:#?}");
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
