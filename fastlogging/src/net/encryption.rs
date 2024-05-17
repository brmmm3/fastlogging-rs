use ring::{ aead, error::Unspecified };

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
        (&mut nonce[0..8]).copy_from_slice(&self.last_nonce.to_le_bytes()); // 0 guarantees that this is correct lmao
        Ok(aead::Nonce::assume_unique_for_key(nonce))
    }
}
