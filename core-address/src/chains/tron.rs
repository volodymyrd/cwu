use crate::{Address, AddressError, AddressResult};
use hex::FromHex;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use serde::Serialize;
use sha2::{Digest, Sha256};
use sha3::Keccak256;
use std::{fmt, str::FromStr};

const ADDRESS_TYPE_PREFIX: u8 = 0x41;

#[derive(Clone, PartialEq, Eq, Serialize, Hash)]
pub struct TronAddress([u8; 21]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TronPublicKey(k256::PublicKey);

/// Represents the format of a Tron address
#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TronFormat {
    Standard,
}

impl fmt::Display for TronFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TronFormat")
    }
}

impl Address for TronAddress {
    type Format = TronFormat;
    type PublicKey = TronPublicKey;

    fn from_public_key(
        public_key: &Self::PublicKey,
        _format: &Self::Format,
    ) -> AddressResult<Self> {
        let mut hasher = Keccak256::new();

        hasher.update(&public_key.0.to_encoded_point(false).as_bytes()[1..]);
        let digest = hasher.finalize();
        let mut raw = [ADDRESS_TYPE_PREFIX; 21];
        raw[1..21].copy_from_slice(&digest[digest.len() - 20..]);

        Ok(TronAddress(raw))
    }
}

impl TronAddress {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn from_bytes(raw: &[u8]) -> AddressResult<Self> {
        if raw.len() != 21 {
            return Err(AddressError::InvalidAddress("Invalid length".to_string()));
        }

        let mut address = [0u8; 21];
        address.copy_from_slice(raw);
        Ok(TronAddress(address))
    }
}

impl fmt::Display for TronAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        b58encode_check(self.0).fmt(f)
    }
}

impl fmt::Debug for TronAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Address").field(&self.to_string()).finish()
    }
}

impl TryFrom<Vec<u8>> for TronAddress {
    type Error = AddressError;

    fn try_from(value: Vec<u8>) -> AddressResult<Self> {
        Self::try_from(&value[..])
    }
}

impl TryFrom<&[u8]> for TronAddress {
    type Error = AddressError;

    fn try_from(value: &[u8]) -> AddressResult<Self> {
        if value.len() != 21 {
            Err(AddressError::InvalidAddress("Invalid length".to_string()))
        } else if value[0] != ADDRESS_TYPE_PREFIX {
            Err(AddressError::Message(format!(
                "Invalid version byte {}",
                value[0]
            )))
        } else {
            let mut raw = [0u8; 21];
            raw[..21].copy_from_slice(value);
            Ok(TronAddress(raw))
        }
    }
}

impl FromStr for TronAddress {
    type Err = AddressError;

    fn from_str(s: &str) -> AddressResult<Self>
    where
        Self: Sized,
    {
        if s.len() == 34 {
            b58decode_check(s).and_then(TronAddress::try_from)
        } else if s.len() == 42 && s[..2] == hex::encode([ADDRESS_TYPE_PREFIX]) {
            Vec::from_hex(s)
                .map_err(|_| AddressError::InvalidAddress("InvalidAddress".to_string()))
                .and_then(TronAddress::try_from)
        } else if s.len() == 44 && (s.starts_with("0x") || s.starts_with("0X")) {
            Vec::from_hex(&s.as_bytes()[2..])
                .map_err(|_| AddressError::InvalidAddress("InvalidAddress".to_string()))
                .and_then(TronAddress::try_from)
        } else if s == "_" || s == "0x0" || s == "/0" {
            "410000000000000000000000000000000000000000".parse()
        } else {
            Err(AddressError::InvalidAddress("Invalid length".to_string()))
        }
    }
}

/// Base58check encode.
pub fn b58encode_check<T: AsRef<[u8]>>(raw: T) -> String {
    let mut hasher = Sha256::new();
    hasher.update(raw.as_ref());
    let digest1 = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(digest1);
    let digest = hasher.finalize();

    let mut raw = raw.as_ref().to_owned();
    raw.extend(&digest[..4]);
    bs58::encode(raw).into_string()
}

/// Base58check decode.
pub fn b58decode_check(s: &str) -> AddressResult<Vec<u8>> {
    let mut result = bs58::decode(s)
        .into_vec()
        .map_err(|_| AddressError::InvalidAddress("".to_string()))?;

    let check = result.split_off(result.len() - 4);

    let mut hasher = Sha256::new();
    hasher.update(&result);
    let digest1 = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(digest1);
    let digest = hasher.finalize();

    if check != digest[..4] {
        Err(AddressError::InvalidAddress("".to_string()))
    } else {
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::ToHex;
    use k256::{EncodedPoint, PublicKey};

    #[test]
    fn test_address() {
        let addr = TronAddress([
            65, 150, 163, 186, 206, 90, 218, 207, 99, 126, 183, 204, 121, 213, 120, 127, 66, 71,
            218, 75, 190,
        ]);

        assert_eq!("TPhiVyQZ5xyvVK2KS2LTke8YvXJU5wxnbN", format!("{:}", addr));
        assert_eq!(
            addr,
            "TPhiVyQZ5xyvVK2KS2LTke8YvXJU5wxnbN"
                .parse()
                .expect("parse error")
        );
        assert_eq!(
            addr,
            "4196a3bace5adacf637eb7cc79d5787f4247da4bbe"
                .parse()
                .expect("parse error")
        );

        assert_eq!(
            addr.as_bytes().encode_hex::<String>(),
            "4196a3bace5adacf637eb7cc79d5787f4247da4bbe"
        )
    }

    #[test]
    fn test_address_from_bytes() {
        let bytes = [
            65, 150, 163, 186, 206, 90, 218, 207, 99, 126, 183, 204, 121, 213, 120, 127, 66, 71,
            218, 75, 190,
        ];
        let addr = TronAddress::from_bytes(&bytes);
        assert!(addr.is_ok());

        let malicious_bytes: [u8; 22] = [
            0xde, 0xad, 0xbe, 0xef, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let addr = TronAddress::from_bytes(&malicious_bytes);
        assert!(addr.is_err());
    }

    #[test]
    fn test_from_public_key_standard() {
        let public_key_hex = "040738c1aa72b07ff1a894198374a34c760913db0e6a5679d48477873b8f1fa865a5923ae7346c3c717579006f9f853adcb7bb5563022775591895253a0457f0df";
        let pk_bytes = hex::decode(public_key_hex).expect("Invalid public key hex");
        let encoded_point =
            EncodedPoint::from_bytes(&pk_bytes).expect("Invalid public key hex for EncodedPoint");
        let public_key = PublicKey::try_from(encoded_point)
            .expect("Failed to create public key from encoded point");
        let tron_public_key = TronPublicKey(public_key);

        let derived_address = TronAddress::from_public_key(&tron_public_key, &TronFormat::Standard)
            .expect("Address derivation failed");

        assert_eq!(
            derived_address.to_string(),
            "TWer2Ygk5TEheHp3TPuYeqxmB6SsGZmaL6".to_string(),
            "Derived Tron Address does not match expected address."
        );
    }
}
