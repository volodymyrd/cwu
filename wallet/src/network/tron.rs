//! # Tron Key Generation
//!
//! This module provides functionality for generating TRON private and public key pairs
//! from a mnemonic phrase, following the BIP39 and BIP44 standards. It uses the TRON
//! designated derivation path `m/44'/195'/0'/0/0`.

use crate::{WalletError, key_pair::KeyPair};
use bip39::Mnemonic;
use k256::{
    PublicKey, SecretKey, elliptic_curve::generic_array::GenericArray,
    elliptic_curve::sec1::ToEncodedPoint,
};
use sha2::Sha256;
use sha3::{Digest, Keccak256};
use tiny_hderive::bip32::ExtendedPrivKey;

const TRON_DERIVATION_PATH: &str = "m/44'/195'/0'/0/0";

/// Generates a TRON key pair from a mnemonic phrase.
pub(crate) fn get_tron_key_pair_from_mnemonic(
    mnemonic: &Mnemonic,
    passphrase: &str,
) -> Result<KeyPair, WalletError> {
    // 1. Mnemonic to Seed
    let seed = mnemonic.to_seed(passphrase);

    // 2. Hierarchical-Deterministic (HD) Wallet derivation
    let key = ExtendedPrivKey::derive(&seed, TRON_DERIVATION_PATH).expect("Valid path");

    // 3. Private and Public Keys
    let secret = key.secret();
    let secret_generic_array = GenericArray::from_slice(&secret);
    let secret_key = SecretKey::from_bytes(secret_generic_array)?;
    let public_key = secret_key.public_key();

    // 4. TRON Address
    let address = public_key_to_tron_address(&public_key);

    Ok(KeyPair::new(hex::encode(secret_key.to_bytes()), address))
}

/// Converts a public key to a TRON address.
///
/// The process is as follows:
/// 1. Get the uncompressed public key (65 bytes, starting with `0x04`).
/// 2. Hash the public key using Keccak-256, and discard the first 12 bytes of the hash.
/// 3. Prepend the TRON address prefix `0x41` to the result.
/// 4. The resulting 21 bytes are the address.
/// 5. Encode this address using Base58Check.
fn public_key_to_tron_address(public_key: &PublicKey) -> String {
    // Get the uncompressed public key and remove the `0x04` prefix.
    let uncompressed_pk = public_key.to_encoded_point(false);
    let public_key_bytes = &uncompressed_pk.as_bytes()[1..];

    // Hash the public key using Keccak-256.
    let mut hasher = Keccak256::new();
    hasher.update(public_key_bytes);
    let hash = hasher.finalize();

    // Construct the address by taking the last 20 bytes of the hash
    // and prepending the TRON address prefix (0x41).
    let mut address_bytes = [0u8; 21];
    address_bytes[0] = 0x41; // TRON address prefix
    address_bytes[1..].copy_from_slice(&hash[12..]);

    // Base58Check encode the address.
    base58check_encode(&address_bytes)
}

/// Encodes a byte slice into a Base58Check string.
///
/// Base58Check encoding is used in TRON (and Bitcoin) to create human-readable
/// addresses with a built-in checksum to prevent typos.
///
/// The process is:
/// 1. Take the input data (e.g., a 21-byte TRON address).
/// 2. Double-hash it with SHA-256: `checksum = sha256(sha256(data))`.
/// 3. Take the first 4 bytes of the checksum.
/// 4. Append these 4 bytes to the original data.
/// 5. Encode the result using the Base58 alphabet.
fn base58check_encode(payload: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(payload);
    let first_hash = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(first_hash);
    let second_hash = hasher.finalize();

    let checksum = &second_hash[0..4];

    let mut checked_payload = Vec::with_capacity(payload.len() + 4);
    checked_payload.extend_from_slice(payload);
    checked_payload.extend_from_slice(checksum);

    bs58::encode(checked_payload).into_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_tron_key_derivation() {
        let mnemonic =
            Mnemonic::from_str("test test test test test test test test test test test junk")
                .unwrap();
        let key_pair = get_tron_key_pair_from_mnemonic(&mnemonic, "").unwrap();

        let expected_private_key =
            "15f0bbb1774be40b7a8d7965d637f324bda2f711fc5726a3dcc19585c6950954";
        let expected_address = "TWer2Ygk5TEheHp3TPuYeqxmB6SsGZmaL6";

        assert_eq!(key_pair.private_key(), expected_private_key);
        assert_eq!(key_pair.address(), expected_address);
    }
}
