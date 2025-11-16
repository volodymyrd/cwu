use crate::Result;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use zeroize::Zeroize;

#[derive(Serialize, Deserialize)]
pub(crate) struct KeyPair {
    private_key: String,
    address: String,
}

impl Zeroize for KeyPair {
    fn zeroize(&mut self) {
        self.private_key.zeroize();
        self.address.zeroize();
    }
}

impl Drop for KeyPair {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl KeyPair {
    pub(crate) fn new(private_key: String, address: String) -> Self {
        Self {
            private_key,
            address,
        }
    }

    pub(crate) fn private_key(&self) -> &str {
        &self.private_key
    }

    pub(crate) fn address(&self) -> &str {
        &self.address
    }

    pub(crate) fn serialize(&self) -> Result<String> {
        Ok(to_string_pretty(&self)?)
    }
}
