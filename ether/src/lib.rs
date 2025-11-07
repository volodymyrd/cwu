//! # ether
//!
//! A Rust library for interacting with the Ethereum blockchain.

pub mod usdt;
pub(crate) mod weth9;

pub(crate) const PUBLIC_RPC_URL: &str = "https://ethereum-rpc.publicnode.com";

#[derive(thiserror::Error, Debug)]
pub enum EtherError {
    #[error("Missing required configuration name: '{0}'")]
    MissingName(String),

    #[error("Invalid RPC URL provided: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Contract error: {0}")]
    AlloyContractError(#[from] alloy::contract::Error),

    #[error("Units error: {0}")]
    AlloyUnitsError(#[from] alloy::primitives::utils::UnitsError),

    #[error("An error occurred: {0}")]
    Err(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl From<Box<dyn std::error::Error + Send + Sync + 'static>> for EtherError {
    fn from(err: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        EtherError::Err(err)
    }
}

type Result<T> = std::result::Result<T, EtherError>;
