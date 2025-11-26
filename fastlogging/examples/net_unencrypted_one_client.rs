use std::{thread, time::Duration};

use fastlogging::{
    ClientWriterConfig, ConsoleWriterConfig, DEBUG, EncryptionMethod, FileWriterConfig, Logging,
    LoggingError, ServerConfig,
};
use tempfile::TempDir;

fn main() -> Result<(), LoggingError> {
    let temp_dir = TempDir::with_prefix("fastlogging").unwrap();
    let log_file = temp_dir.path().join("file.log");
    // Server
    let mut logging_server = Logging::new(
        DEBUG,
        "LOGSRV",
        Some(vec![
            ConsoleWriterConfig::new(DEBUG, true).into(),
            FileWriterConfig::new(DEBUG, log_file.clone(), 0, 0, None, None, None)
                .unwrap()
                .into(),
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
    // Client
    let mut logging_client = Logging::new(
        DEBUG,
        "LOGCLIENT",
        Some(vec![
            ClientWriterConfig::new(
                DEBUG,
                logging_server.get_root_server_address_port().unwrap(),
                logging_server.get_server_auth_key(),
            )
            .into(),
            //ConsoleWriterConfig::new(DEBUG, false).into()
        ]),
        None,
        None,
    )?;
    //logging_client.set_debug(3);
    println!("Send logs");
    logging_client.trace("Trace Message".to_string()).unwrap();
    logging_client.debug("Debug Message".to_string()).unwrap();
    logging_client.info("Info Message".to_string()).unwrap();
    logging_client
        .success("Success Message".to_string())
        .unwrap();
    logging_client
        .warning("Warning Message".to_string())
        .unwrap();
    logging_client.error("Error Message".to_string()).unwrap();
    logging_client.fatal("Fatal Message".to_string()).unwrap();

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

    logging_client.sync_all(1.0)?;
    logging_server.sync_all(1.0)?;
    // Give client some time to send the log messages
    thread::sleep(Duration::from_millis(50));
    println!("Shutdown Loggers");
    logging_client.shutdown(false).unwrap();
    logging_server.shutdown(false).unwrap();
    let _log_text = std::fs::read_to_string(&log_file).unwrap();
    temp_dir.close().unwrap();
    println!("-------- Finished --------");
    Ok(())
}
