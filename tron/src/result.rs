#[derive(thiserror::Error, Debug)]
pub enum TronError {
    #[error("Tronic error: {0}")]
    TronicError(#[from] tronic::error::Error),

    #[error("Error: {0}")]
    Error(#[from] anyhow::Error),

    #[error("An error occurred: {0}")]
    Err(Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("USDT smart contract address is not set")]
    UsdtSmartContractAddressIsNotSet,
}

impl From<Box<dyn std::error::Error + Send + Sync + 'static>> for TronError {
    fn from(err: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        TronError::Err(err)
    }
}

pub type Result<T> = std::result::Result<T, TronError>;
