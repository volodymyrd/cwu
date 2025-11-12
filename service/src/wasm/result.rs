use thiserror::Error;

#[derive(Debug, Error)]
pub enum WasmError {
    #[error("{0}")]
    AnyhowError(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, WasmError>;
