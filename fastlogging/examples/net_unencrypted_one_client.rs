use std::{ io::Error, thread, time::Duration };

use fastlogging::{
    ClientWriterConfig,
    ConsoleWriterConfig,
    EncryptionMethod,
    FileWriterConfig,
    Logging,
    ServerConfig,
    DEBUG,
};
use tempdir::TempDir;

fn main() -> Result<(), Error> {
    let temp_dir = TempDir::new("fastlogging").unwrap();
    let log_file = temp_dir.path().join("file.log");
    let console_writer = ConsoleWriterConfig::new(DEBUG, true);
    let file_writer = FileWriterConfig::new(
        DEBUG,
        log_file.clone(),
        0,
        0,
        None,
        None,
        None
    ).unwrap();
    let server_config = ServerConfig::new(DEBUG, "127.0.0.1", EncryptionMethod::None);
    let mut logging_server = Logging::new(
        None,
        Some("LOGSRV".to_string()),
        None,
        Some(console_writer),
        Some(file_writer),
        Some(server_config),
        None,
        None
    ).unwrap();
    logging_server.sync_all(5.0).unwrap();
    //let console_writer2 = ConsoleWriterConfig::new(DEBUG, false);
    let client_writer = ClientWriterConfig::new(
        DEBUG,
        format!("127.0.0.1:{}", logging_server.get_server_config().unwrap().port),
        EncryptionMethod::AuthKey(logging_server.get_server_auth_key())
    );
    let mut logging_client = Logging::new(
        None,
        Some("LOGCLIENT".to_string()),
        None,
        None, //Some(console_writer2),
        None,
        None,
        Some(client_writer),
        None
    ).unwrap();
    println!("Send logs");
    logging_client.trace("Trace Message".to_string()).unwrap();
    logging_client.debug("Debug Message".to_string()).unwrap();
    logging_client.info("Info Message".to_string()).unwrap();
    logging_client.success("Success Message".to_string()).unwrap();
    logging_client.warning("Warning Message".to_string()).unwrap();
    logging_client.error("Error Message".to_string()).unwrap();
    logging_client.fatal("Fatal Message".to_string()).unwrap();
    logging_client.sync_all(1.0)?;
    logging_server.sync_all(1.0)?;
    // Give client some time to send the log messages
    thread::sleep(Duration::from_millis(50));
    println!("Shutdown Loggers");
    logging_client.shutdown(false).unwrap();
    logging_server.shutdown(false).unwrap();
    let log_text = std::fs::read_to_string(&log_file).unwrap();
    temp_dir.close().unwrap();
    println!("-------- Finished --------");
    Ok(())
}
