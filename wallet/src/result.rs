use crate::language::InvalidLanguage;
use cwu_model::Network;
use cwu_security_utils::{EncryptionError, PasswordError};
use k256::elliptic_curve;

#[derive(thiserror::Error, Debug)]
pub enum WalletError {
    #[error("{0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("{0}")]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    Bip39Error(#[from] bip39::Error),

    #[error("Could not parse mnemonic language")]
    InvalidLanguageParse(#[from] InvalidLanguage),

    #[error("{0}")]
    EncryptionError(#[from] EncryptionError),

    #[error("{0}")]
    PasswordError(#[from] PasswordError),

    #[error("{0}")]
    EllipticCurveError(#[from] elliptic_curve::Error),

    #[error("{0}")]
    UnsupportedVersion(u32),

    #[error("Not found key pair for network: {0}")]
    NotFoundKeyPair(Network),
}

pub type Result<T> = std::result::Result<T, WalletError>;
