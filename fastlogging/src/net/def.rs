use std::{ fmt, io::{ Error, ErrorKind } };

use ring::aead::{ self, BoundKey, SealingKey };
use once_cell::sync::Lazy;
use rand::{ distributions::Alphanumeric, thread_rng, Rng };

use super::NonceGenerator;

pub static AUTH_KEY: Lazy<Vec<u8>> = Lazy::new(|| {
    thread_rng().sample_iter(&Alphanumeric).take(32).collect()
});

#[derive(Debug)]
pub struct NetConfig {
    pub level: u8,
    pub address: String,
    pub key: Option<Vec<u8>>,
    pub sk: Option<SealingKey<NonceGenerator>>,
    pub seal: String,
}

impl NetConfig {
    pub fn new(level: u8, address: String, key: Option<Vec<u8>>) -> Result<Self, Error> {
        let mut config = Self {
            level,
            address,
            key: None,
            sk: None,
            seal: "FastLoggingRs".to_string(),
        };
        config.set_encryption(key)?;
        Ok(config)
    }

    pub fn set_encryption(&mut self, key: Option<Vec<u8>>) -> Result<(), Error> {
        if let Some(key) = key {
            self.sk = Some(
                aead::SealingKey::new(
                    aead::UnboundKey
                        ::new(&aead::AES_256_GCM, &key)
                        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?,
                    NonceGenerator::new()
                )
            );
            self.key = Some(key);
        } else {
            self.sk = None;
            self.key = None;
        }
        Ok(())
    }
}

impl fmt::Display for NetConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
