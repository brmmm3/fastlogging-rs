use std::path::PathBuf;

use wasm_bindgen::prelude::*;

use crate::Logger;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum LevelSyms {
    Sym = 0,
    Short = 1,
    Str = 2,
}

#[wasm_bindgen]
pub struct Logging(Box<fastlogging::Logging>);

#[wasm_bindgen]
impl Logging {
    #[wasm_bindgen(constructor)]
    pub fn new(
        level: Option<u8>, // Global log level
        domain: Option<String>,
        console: Option<bool>, // If true start ConsoleLogging
        file: Option<String>, // If path is defined start FileLogging
        server: Option<String>, // If address is defined start LoggingServer
        connect: Option<String>, // If address is defined start ClientLogging
        max_size: Option<usize>, // Maximum size of log files
        backlog: Option<usize> // Maximum number of backup log files
    ) -> Result<Logging, JsError> {
        Ok(
            Logging(
                Box::new(
                    fastlogging::Logging::new(
                        level,
                        domain,
                        console,
                        file.map(|f| PathBuf::from(f)),
                        server,
                        connect,
                        max_size,
                        backlog
                    )?
                )
            )
        )
    }

    pub fn init() -> Result<Logging, JsError> {
        Ok(
            Logging(
                Box::new(
                    fastlogging::Logging::new(None, None, Some(true), None, None, None, None, None)?
                )
            )
        )
    }

    pub fn shutdown(&mut self, now: Option<bool>) -> Result<(), JsError> {
        self.0.shutdown(now).map_err(|e| e.into())
    }

    pub fn add_logger(&mut self, logger: &mut Logger) {
        logger.set_tx(Some(self.0.tx.clone()));
    }

    pub fn remove_logger(&mut self, logger: &mut Logger) {
        logger.set_tx(None);
    }

    pub fn set_level(&mut self, level: u8) {
        self.0.set_level(level);
    }

    pub fn set_domain(&mut self, domain: String) {
        self.0.set_domain(domain);
    }

    pub fn set_level2sym(&mut self, level2sym: LevelSyms) {
        self.0.set_level2sym(match level2sym {
            LevelSyms::Sym => fastlogging::LevelSyms::Sym,
            LevelSyms::Short => fastlogging::LevelSyms::Short,
            LevelSyms::Str => fastlogging::LevelSyms::Str,
        });
    }

    // Console logger

    pub fn set_console_writer(&mut self, level: Option<u8>) -> Result<(), JsError> {
        self.0.set_console_writer(level).map_err(|e| e.into())
    }

    pub fn set_console_colors(&mut self, colors: bool) {
        self.0.set_console_colors(colors);
    }

    // File logger

    pub fn set_file_writer(
        &mut self,
        level: Option<u8>,
        path: Option<String>,
        max_size: Option<usize>, // Maximum size of log files
        backlog: Option<usize> // Maximum number of backup log files
    ) -> Result<(), JsError> {
        self.0
            .set_file_writer(
                level,
                path.map(|p| PathBuf::from(p)),
                max_size,
                backlog
            )
            .map_err(|e| e.into())
    }

    pub fn rotate(&self) -> Result<(), JsError> {
        self.0.rotate().map_err(|e| e.into())
    }

    pub fn sync(&self, timeout: f64) -> Result<(), JsError> {
        self.0.sync(timeout).map_err(|e| e.into())
    }

    // Network client

    pub fn connect(
        &mut self,
        address: String,
        level: u8,
        key: Option<Vec<u8>>
    ) -> Result<(), JsError> {
        self.0.connect(address, level, key).map_err(|e| e.into())
    }

    pub fn disconnect(&mut self, address: &str) -> Result<(), JsError> {
        self.0.disconnect(address).map_err(|e| e.into())
    }

    pub fn set_client_level(&mut self, address: &str, level: u8) -> Result<(), JsError> {
        self.0.set_client_level(address, level).map_err(|e| e.into())
    }

    pub fn set_client_encryption(
        &mut self,
        address: &str,
        key: Option<Vec<u8>>
    ) -> Result<(), JsError> {
        self.0.set_client_encryption(address, key).map_err(|e| e.into())
    }

    // Network server

    pub fn server_start(
        &mut self,
        address: String,
        level: u8,
        key: Option<Vec<u8>>
    ) -> Result<(), JsError> {
        self.0.server_start(address, level, key).map_err(|e| e.into())
    }

    pub fn server_shutdown(&mut self) -> Result<(), JsError> {
        self.0.server_shutdown().map_err(|e| e.into())
    }

    pub fn set_server_level(&mut self, level: u8) -> Result<(), JsError> {
        self.0.set_server_level(level).map_err(|e| e.into())
    }

    pub fn set_server_encryption(&mut self, key: Option<Vec<u8>>) -> Result<(), JsError> {
        self.0.set_server_encryption(key).map_err(|e| e.into())
    }

    // Logging calls

    pub fn debug(&self, message: String) -> Result<(), JsError> {
        self.0.debug(message).map_err(|e| e.into())
    }

    pub fn info(&self, message: String) -> Result<(), JsError> {
        self.0.info(message).map_err(|e| e.into())
    }

    pub fn warning(&self, message: String) -> Result<(), JsError> {
        self.0.warning(message).map_err(|e| e.into())
    }

    pub fn error(&self, message: String) -> Result<(), JsError> {
        self.0.error(message).map_err(|e| e.into())
    }

    pub fn critical(&self, message: String) -> Result<(), JsError> {
        self.0.critical(message).map_err(|e| e.into())
    }

    pub fn fatal(&self, message: String) -> Result<(), JsError> {
        self.0.fatal(message).map_err(|e| e.into())
    }

    pub fn exception(&self, message: String) -> Result<(), JsError> {
        self.0.exception(message).map_err(|e| e.into())
    }
}
