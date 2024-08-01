use std::{
    fmt,
    io::{Error, ErrorKind},
    path::PathBuf,
};

use once_cell::sync::Lazy;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use ring::aead::{self, BoundKey, SealingKey};

use crate::{ClientWriterConfig, ServerConfig};

use super::{EncryptionMethod, NonceGenerator};

pub static AUTH_KEY: Lazy<Vec<u8>> =
    Lazy::new(|| thread_rng().sample_iter(&Alphanumeric).take(32).collect());

#[derive(Debug)]
pub struct NetConfig {
    pub level: u8,
    pub address: String,
    pub port: u16,
    pub key: EncryptionMethod,
    pub sk: Option<SealingKey<NonceGenerator>>,
    pub seal: String,
    pub port_file: Option<PathBuf>,
    pub debug: u8,
}

impl NetConfig {
    pub fn new(
        level: u8,
        address: String,
        port: u16,
        key: EncryptionMethod,
    ) -> Result<Self, Error> {
        let mut config = Self {
            level,
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

    pub fn set_encryption(&mut self, key: EncryptionMethod) -> Result<(), Error> {
        self.key = key.clone();
        match &key {
            EncryptionMethod::AES(key) => {
                self.sk = Some(aead::SealingKey::new(
                    aead::UnboundKey::new(&aead::AES_256_GCM, key)
                        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?,
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
            level: self.level,
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
