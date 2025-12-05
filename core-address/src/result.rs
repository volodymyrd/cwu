use thiserror::Error;

pub type AddressResult<T> = std::result::Result<T, AddressError>;

#[derive(Debug, Error)]
pub enum AddressError {
    #[error("invalid address: {0:}")]
    InvalidAddress(String),
    #[error("{0}")]
    Message(String),
}
