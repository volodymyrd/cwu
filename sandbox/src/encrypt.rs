use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit, Payload},
};
use argon2::{
    Argon2, Params, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use base64::prelude::*;
use rand::{RngCore, thread_rng};

const KEY_SIZE: usize = 32; // 256 bits for AES-256
const NONCE_SIZE: usize = 12;

/// A struct to hold the encrypted data and associated metadata.
#[derive(Debug, Clone)]
pub struct EncryptedPayload {
    pub ciphertext_b64: String,
    pub salt_phc: String,
    pub nonce_b64: String,
}

/// Derives a 32-byte (256-bit) key from a master password and salt using Argon2id.
fn derive_key(master_pass: &str, salt: &SaltString) -> Result<[u8; KEY_SIZE], String> {
    // Define Argon2 parameters for key derivation (not just password hashing)
    // Argon2id is the recommended variant (hybrid of 'i' and 'd').
    let params = Params::new(
        19456,          // m_cost (memory cost)
        2,              // t_cost (time cost/iterations)
        1,              // p_cost (parallelism)
        Some(KEY_SIZE), // Output key length
    )
    .map_err(|e| format!("Argon2 parameter error: {:?}", e))?;

    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

    // Hash the combined password to get the derived key
    // The hash output is the key material.
    let hash_output = argon2
        .hash_password(master_pass.as_bytes(), salt)
        .map_err(|e| format!("Argon2 hashing error: {:?}", e))?;

    // Extract the raw 32-byte key from the hash structure
    let key_slice = hash_output
        .hash
        .as_ref()
        .and_then(|h| h.as_bytes().get(..KEY_SIZE))
        .ok_or_else(|| "Failed to extract key of correct size".to_string())?;

    let mut derived_key = [0u8; KEY_SIZE];
    derived_key.copy_from_slice(key_slice);

    Ok(derived_key)
}

/// Encrypts a string using AES-256-GCM, deriving the key from a master password.
///
/// Returns an `EncryptedPayload` struct containing the encrypted data and metadata.
pub fn encrypt(plaintext: &str, master_pass: &str) -> Result<EncryptedPayload, String> {
    // 1. Generate a secure, unique KDF Salt for Argon2
    let salt = SaltString::generate(&mut OsRng);

    // 2. Derive the 32-byte AES key from the master password and salt
    let key_bytes = derive_key(master_pass, &salt)?;
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    // 3. Generate a secure 12-byte Nonce for AES-GCM
    let mut rng = thread_rng();
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    rng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

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
        Err(e) => Err(format!("Encryption error: {:?}", e)),
    }
}

/// Decrypts an `EncryptedPayload` using AES-256-GCM, deriving the key from a master password.
pub fn decrypt(
    payload: &EncryptedPayload,
    master_pass: &str,
) -> Result<String, String> {
    // 1. Decode Base64 inputs
    let ciphertext_with_tag = BASE64_STANDARD
        .decode(&payload.ciphertext_b64)
        .map_err(|e| format!("Base64 decode error for ciphertext: {:?}", e))?;
    let nonce_bytes = BASE64_STANDARD
        .decode(&payload.nonce_b64)
        .map_err(|e| format!("Base64 decode error for nonce: {:?}", e))?;

    // 2. Parse the KDF salt string back into a SaltString
    let salt = SaltString::new(&payload.salt_phc)
        .map_err(|e| format!("Failed to parse KDF salt string: {:?}", e))?;

    // 3. Derive the 32-byte AES key from the master password and salt
    let key_bytes = derive_key(master_pass, &salt)?;
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    // 4. Create Nonce
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 5. Decryption (AES-256-GCM)
    let cipher = Aes256Gcm::new(key);

    match cipher.decrypt(nonce, Payload { msg: &ciphertext_with_tag, aad: &[] }) {
        Ok(plaintext_bytes) => String::from_utf8(plaintext_bytes)
            .map_err(|e| format!("UTF-8 decoding error: {:?}", e)),
        Err(e) => Err(format!("Decryption error: {:?}", e)),
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

        println!("--- Original Text ---");
        println!("{}", original_text);
        println!("\n--- Encryption Result (AES-256-GCM + Argon2id KDF) ---");
        println!("Ciphertext (B64): {}", payload.ciphertext_b64);
        println!("Argon2 Salt/Params (PHC String): {}", payload.salt_phc);
        println!("Nonce/IV (B64): {}", payload.nonce_b64);

        // Decrypt
        let decrypted_text = decrypt(&payload, master_pass).expect("Decryption failed");

        println!("\n--- Decryption Result ---");
        println!("{}", decrypted_text);

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
        assert!(result.unwrap_err().contains("Decryption error"));
    }

    #[test]
    fn test_encrypt_different_nonce_fail() {
        let original_text = "This is another secret message.";
        let master_pass = "MyPassword";

        let mut payload = encrypt(original_text, master_pass).expect("Encryption failed");

        // Generate a different nonce
        let mut rng = thread_rng();
        let mut different_nonce_bytes = [0u8; NONCE_SIZE];
        rng.fill_bytes(&mut different_nonce_bytes);
        payload.nonce_b64 = BASE64_STANDARD.encode(different_nonce_bytes);

        let result = decrypt(&payload, master_pass);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Decryption error"));
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
        assert!(result.unwrap_err().contains("Decryption error"));
    }
}
