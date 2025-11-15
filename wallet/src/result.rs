use crate::language::InvalidLanguage;
use cwu_security_utils::{EncryptionError, PasswordError};

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
}

pub type Result<T> = std::result::Result<T, WalletError>;
