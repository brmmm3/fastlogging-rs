use ring::{aead, error::Unspecified};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionMethod {
    NONE,
    AuthKey(Vec<u8>),
    AES(Vec<u8>),
}

impl EncryptionMethod {
    pub fn is_encrypted(&self) -> bool {
        matches!(self, Self::AES(_))
    }

    pub fn key(&self) -> Option<&[u8]> {
        match self {
            Self::NONE => None,
            Self::AuthKey(key) => Some(key),
            Self::AES(key) => Some(key),
        }
    }

    pub fn key_cloned(&self) -> Option<Vec<u8>> {
        match self {
            Self::NONE => None,
            Self::AuthKey(key) => Some(key.to_vec()),
            Self::AES(key) => Some(key.to_vec()),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::NONE => 0,
            Self::AuthKey(key) => key.len(),
            Self::AES(key) => key.len(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            Self::NONE => {
                bytes.push(0);
            }
            Self::AuthKey(key) => {
                bytes.push(1);
                bytes.extend(key);
            }
            Self::AES(key) => {
                bytes.push(2);
                bytes.extend(key);
            }
        }
        bytes
    }
}

#[derive(Debug)]
pub struct NonceGenerator {
    last_nonce: u64,
}

impl NonceGenerator {
    pub fn new() -> Self {
        Self { last_nonce: 0 }
    }
}

impl aead::NonceSequence for NonceGenerator {
    fn advance(&mut self) -> Result<aead::Nonce, Unspecified> {
        self.last_nonce += self.last_nonce.checked_add(1).ok_or(Unspecified)?;
        let mut nonce = [0u8; 12];
        nonce[0..8].copy_from_slice(&self.last_nonce.to_le_bytes()); // 0 guarantees that this is correct lmao
        Ok(aead::Nonce::assume_unique_for_key(nonce))
    }
}
