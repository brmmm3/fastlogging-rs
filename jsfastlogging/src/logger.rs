use fastlogging::MessageType;
use flume::Sender;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Logger(Box<fastlogging::Logger>);

impl Logger {
    pub fn set_tx(&mut self, tx: Option<Sender<MessageType>>) {
        self.0.set_tx(tx);
    }
}

#[wasm_bindgen]
impl Logger {
    #[wasm_bindgen(constructor)]
    pub fn new(level: u8, domain: String) -> Logger {
        Logger(Box::new(fastlogging::Logger::new(level, domain)))
    }

    pub fn set_level(&mut self, level: u8) {
        self.0.set_level(level);
    }

    pub fn set_domain(&mut self, domain: String) {
        self.0.set_domain(domain);
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
