use crate::wasm::WasmError;
use cwu_ether::EtherError;
use cwu_tron::TronError;

#[derive(thiserror::Error, Debug)]
pub enum CwuServiceError {
    #[error("Address not found")]
    AddressNotFound,
    #[error("{0}")]
    EtherError(#[from] EtherError),
    #[error("{0}")]
    TronError(#[from] TronError),
    #[error("{0}")]
    AnyhowError(#[from] anyhow::Error),
    #[error("{0}")]
    WasmError(#[from] WasmError),
}

pub type Result<T> = std::result::Result<T, CwuServiceError>;
