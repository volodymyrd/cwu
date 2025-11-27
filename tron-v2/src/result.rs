#[derive(thiserror::Error, Debug)]
pub enum TronError {}

pub type Result<T> = std::result::Result<T, TronError>;
