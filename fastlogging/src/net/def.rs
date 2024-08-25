use std::{fmt, path::PathBuf};

use once_cell::sync::Lazy;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use ring::aead::{self, BoundKey, SealingKey};

use crate::{ClientWriterConfig, LoggingError, ServerConfig};

use super::{EncryptionMethod, NonceGenerator};

pub static AUTH_KEY: Lazy<Vec<u8>> =
    Lazy::new(|| thread_rng().sample_iter(&Alphanumeric).take(32).collect());

#[derive(Debug)]
pub struct NetConfig {
    pub(crate) enabled: bool,
    pub(crate) level: u8,
    pub(crate) domain_filter: Option<String>,
    pub(crate) message_filter: Option<String>,
    pub(crate) address: String,
    pub(crate) port: u16,
    pub(crate) key: EncryptionMethod,
    pub(crate) sk: Option<SealingKey<NonceGenerator>>,
    pub(crate) seal: String,
    pub(crate) port_file: Option<PathBuf>,
    pub(crate) debug: u8,
}

impl NetConfig {
    pub fn new(
        level: u8,
        address: String,
        port: u16,
        key: EncryptionMethod,
    ) -> Result<Self, LoggingError> {
        let mut config = Self {
            enabled: true,
            level,
            domain_filter: None,
            message_filter: None,
            address,
            port,
            key: key.clone(),
            sk: None,
            seal: "FastLoggingRs".to_string(),
            port_file: None,
            debug: 0,
        };
        config.set_encryption(key)?;
        Ok(config)
    }

    pub fn set_encryption(&mut self, key: EncryptionMethod) -> Result<(), LoggingError> {
        self.key = key.clone();
        match &key {
            EncryptionMethod::AES(key_vec) => {
                self.sk = Some(aead::SealingKey::new(
                    aead::UnboundKey::new(&aead::AES_256_GCM, key_vec).map_err(|e| {
                        LoggingError::InvalidEncryption("NetConfig".to_string(), key, e.to_string())
                    })?,
                    NonceGenerator::new(),
                ));
            }
            EncryptionMethod::AuthKey(_) => {
                self.sk = None;
            }
            EncryptionMethod::NONE => {
                self.key = EncryptionMethod::AuthKey(AUTH_KEY.to_vec());
                self.sk = None;
            }
        }
        Ok(())
    }

    pub fn get_address(&self) -> String {
        if self.address.contains(':') {
            self.address.clone()
        } else {
            format!("{}:{}", self.address, self.port)
        }
    }

    pub fn get_server_config(&self) -> ServerConfig {
        ServerConfig {
            level: self.level,
            address: self.address.clone(),
            port: self.port,
            key: self.key.clone(),
            port_file: self.port_file.clone(),
        }
    }

    pub fn get_client_config(&self) -> ClientWriterConfig {
        ClientWriterConfig {
            enabled: self.enabled,
            level: self.level,
            domain_filter: self.domain_filter.clone(),
            message_filter: self.message_filter.clone(),
            address: self.address.clone(),
            port: self.port,
            key: self.key.clone(),
            debug: self.debug,
        }
    }
}

impl fmt::Display for NetConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
