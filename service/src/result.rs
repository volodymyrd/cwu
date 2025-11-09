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
}

pub type Result<T> = std::result::Result<T, CwuServiceError>;
