use alloy::hex::FromHexError;

#[derive(thiserror::Error, Debug)]
pub enum EtherError {
    #[error("Invalid RPC URL provided: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Contract error: {0}")]
    AlloyContractError(#[from] alloy::contract::Error),

    #[error("Units error: {0}")]
    AlloyUnitsError(#[from] alloy::primitives::utils::UnitsError),

    #[error("Units error: {0}")]
    HexError(#[from] FromHexError),

    #[error("An error occurred: {0}")]
    Err(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl From<Box<dyn std::error::Error + Send + Sync + 'static>> for EtherError {
    fn from(err: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        EtherError::Err(err)
    }
}

pub type Result<T> = std::result::Result<T, EtherError>;
