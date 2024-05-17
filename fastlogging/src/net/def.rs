use ring::{ aead::{ self, BoundKey, SealingKey }, error::Unspecified };

use super::NonceGenerator;

#[derive(Debug)]
pub struct Config {
    pub level: u8,
    pub address: String,
    pub key: Option<Vec<u8>>,
    pub sk: Option<SealingKey<NonceGenerator>>,
    pub seal: String,
}

impl Config {
    pub fn new(level: u8, address: String) -> Self {
        Self {
            level,
            address,
            key: None,
            sk: None,
            seal: "FastLogging".to_string(),
        }
    }

    pub fn set_encryption(&mut self, key: Option<Vec<u8>>) -> Result<(), Unspecified> {
        if let Some(key) = key {
            if key.starts_with(b"ssh-rsa ") {
            } else {
                self.sk = Some(
                    aead::SealingKey::new(
                        aead::UnboundKey::new(&aead::AES_256_GCM, &key)?,
                        NonceGenerator::new()
                    )
                );
            }
            self.key = Some(key);
        } else {
            self.sk = None;
            self.key = None;
        }
        Ok(())
    }
}
