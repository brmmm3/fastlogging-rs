mod def;
pub use def::AUTH_KEY;
mod server;
pub use server::{LoggingServer, ServerConfig};
mod client;
pub use client::{ClientTypeEnum, ClientWriter, ClientWriterConfig};
mod encryption;
pub use encryption::{EncryptionMethod, NonceGenerator};

#[cfg(test)]
mod tests {
    use tempdir::TempDir;

    use crate::{
        ClientWriterConfig, ConsoleWriterConfig, FileWriterConfig, Logging, ServerConfig, DEBUG,
        NOTSET,
    };

    use super::EncryptionMethod;

    #[test]
    fn unencrypted_one_client() {
        let temp_dir = TempDir::new("fastlogging").unwrap();
        let log_file = temp_dir.path().join("file.log");
        // Server
        let mut logging_server = Logging::new(
            NOTSET,
            "server".to_string(),
            vec![
                ConsoleWriterConfig::new(DEBUG, true).into(),
                FileWriterConfig::new(DEBUG, log_file.clone(), 0, 0, None, None, None)
                    .unwrap()
                    .into(),
            ],
            None,
            None,
        )
        .unwrap();
        logging_server
            .set_root_writer_config(
                &ServerConfig::new(DEBUG, "127.0.0.1", EncryptionMethod::NONE).into(),
            )
            .unwrap();
        logging_server.sync_all(5.0).unwrap();
        // Client
        let mut logging_client = Logging::new(
            NOTSET,
            "client".to_string(),
            vec![
                ConsoleWriterConfig::new(DEBUG, false).into(),
                ClientWriterConfig::new(
                    DEBUG,
                    logging_server.get_server_addresses_ports().get(&0).unwrap(),
                    logging_server.get_server_auth_key(),
                )
                .into(),
            ],
            None,
            None,
        )
        .unwrap();
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
        logging_client.shutdown(false).unwrap();
        logging_server.shutdown(false).unwrap();
        let _log_text = std::fs::read_to_string(&log_file).unwrap();
        temp_dir.close().unwrap();
    }
}
