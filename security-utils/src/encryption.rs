use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, KeyInit, Payload},
};
use argon2::{
    Argon2, Params, PasswordHasher,
    password_hash::{self, SaltString, rand_core::OsRng},
};
use base64::{DecodeError, prelude::*};
use rand::{RngCore, rng};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, fs, path::Path, string::FromUtf8Error};

const KEY_SIZE: usize = 32; // 256 bits for AES-256
const NONCE_SIZE: usize = 12;

/// A struct to hold the encrypted data and associated metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedPayload {
    pub ciphertext_b64: String,
    pub salt_phc: String,
    pub nonce_b64: String,
}

impl EncryptedPayload {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let file_content = fs::read_to_string(path)?;
        let payload: Self = serde_json::from_str(&file_content)?;
        Ok(payload)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum EncryptionError {
    #[error("{0}")]
    ArgonError(#[from] argon2::Error),

    #[error("{0}")]
    ConvertError(#[from] Infallible),

    #[error("{0}")]
    FromUtf8Error(#[from] FromUtf8Error),

    #[error("{0}")]
    DecodeError(#[from] DecodeError),

    #[error("{0}")]
    PasswordHashError(#[from] password_hash::Error),

    #[error("{0}")]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("{0}")]
    Error(String),
}

pub type Result<T> = std::result::Result<T, EncryptionError>;

/// Derives a 32-byte (256-bit) key from a master password and salt using Argon2id.
fn derive_key(master_pass: &str, salt: &SaltString) -> Result<[u8; KEY_SIZE]> {
    // Define Argon2 parameters for key derivation (not just password hashing)
    // Argon2id is the recommended variant (hybrid of 'i' and 'd').
    let params = Params::new(
        19456,          // m_cost (memory cost)
        2,              // t_cost (time cost/iterations)
        1,              // p_cost (parallelism)
        Some(KEY_SIZE), // Output key length
    )?;

    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

    // Hash the combined password to get the derived key
    // The hash output is the key material.
    let hash_output = argon2.hash_password(master_pass.as_bytes(), salt)?;

    // Extract the raw 32-byte key from the hash structure
    let key_slice = hash_output
        .hash
        .as_ref()
        .and_then(|h| h.as_bytes().get(..KEY_SIZE))
        .ok_or_else(|| {
            EncryptionError::Error("Failed to extract key of correct size".to_string())
        })?;

    let mut derived_key = [0u8; KEY_SIZE];
    derived_key.copy_from_slice(key_slice);

    Ok(derived_key)
}

/// Encrypts a string using AES-256-GCM, deriving the key from a master password.
///
/// Returns an `EncryptedPayload` struct containing the encrypted data and metadata.
pub fn encrypt(plaintext: &str, master_pass: &str) -> Result<EncryptedPayload> {
    // 1. Generate a secure, unique KDF Salt for Argon2
    let salt = SaltString::generate(&mut OsRng);

    // 2. Derive the 32-byte AES key from the master password and salt
    let key_bytes = derive_key(master_pass, &salt)?;
    let key = key_bytes.as_slice().into();

    // 3. Generate a secure 12-byte Nonce for AES-GCM
    let mut rng = rng();
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    rng.fill_bytes(&mut nonce_bytes);
    let nonce = nonce_bytes.as_slice().into();

    // 4. Encryption (AES-256-GCM)
    let cipher = Aes256Gcm::new(key);

    match cipher.encrypt(nonce, plaintext.as_bytes()) {
        Ok(ciphertext_with_tag) => {
            // 5. Base64 Encoding and Output
            let ciphertext_b64 = BASE64_STANDARD.encode(&ciphertext_with_tag);
            let nonce_b64 = BASE64_STANDARD.encode(nonce_bytes);
            let salt_phc = salt.to_string();

            Ok(EncryptedPayload {
                ciphertext_b64,
                salt_phc,
                nonce_b64,
            })
        }
        Err(e) => Err(EncryptionError::Error(format!("Encryption error: {:?}", e))),
    }
}

/// Decrypts an `EncryptedPayload` using AES-256-GCM, deriving the key from a master password.
pub fn decrypt(payload: &EncryptedPayload, master_pass: &str) -> Result<String> {
    // 1. Decode Base64 inputs
    let ciphertext_with_tag = BASE64_STANDARD.decode(&payload.ciphertext_b64)?;
    let nonce_bytes = BASE64_STANDARD.decode(&payload.nonce_b64)?;

    // 2. Parse the KDF salt string back into a SaltString
    let salt = SaltString::from_b64(&payload.salt_phc)?;

    // 3. Derive the 32-byte AES key from the master password and salt
    let key_bytes = derive_key(master_pass, &salt)?;
    let key = key_bytes.as_slice().into();

    // 4. Create Nonce
    let nonce = nonce_bytes.as_slice().into();

    // 5. Decryption (AES-256-GCM)
    let cipher = Aes256Gcm::new(key);

    match cipher.decrypt(
        nonce,
        Payload {
            msg: &ciphertext_with_tag,
            aad: &[],
        },
    ) {
        Ok(plaintext_bytes) => Ok(String::from_utf8(plaintext_bytes)?),
        Err(e) => Err(EncryptionError::Error(format!("Decryption error: {:?}", e))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let original_text = "The treasure is buried under the old oak tree.";
        let master_pass = "MySecretPassphrase";

        // Encrypt
        let payload = encrypt(original_text, master_pass).expect("Encryption failed");

        // Decrypt
        let decrypted_text = decrypt(&payload, master_pass).expect("Decryption failed");

        assert_eq!(original_text, decrypted_text);
    }

    #[test]
    fn test_encrypt_different_passwords_fail() {
        let original_text = "This is a secret message.";
        let master_pass = "CorrectPassword";
        let wrong_pass = "IncorrectPassword";

        let payload = encrypt(original_text, master_pass).expect("Encryption failed");

        let result = decrypt(&payload, wrong_pass);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Decryption error: Error".to_string()
        );
    }

    #[test]
    fn test_encrypt_different_nonce_fail() {
        let original_text = "This is another secret message.";
        let master_pass = "MyPassword";

        let mut payload = encrypt(original_text, master_pass).expect("Encryption failed");

        // Generate a different nonce
        let mut rng = rng();
        let mut different_nonce_bytes = [0u8; NONCE_SIZE];
        rng.fill_bytes(&mut different_nonce_bytes);
        payload.nonce_b64 = BASE64_STANDARD.encode(different_nonce_bytes);

        let result = decrypt(&payload, master_pass);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Decryption error: Error");
    }

    #[test]
    fn test_encrypt_different_salt_fail() {
        let original_text = "Yet another secret.";
        let master_pass = "SuperSecret";

        let mut payload = encrypt(original_text, master_pass).expect("Encryption failed");

        // Generate a different salt
        let different_salt = SaltString::generate(&mut OsRng);
        payload.salt_phc = different_salt.to_string();

        let result = decrypt(&payload, master_pass);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Decryption error: Error".to_string()
        );
    }
}
