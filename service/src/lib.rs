mod balance;
mod network;
mod result;
mod service;
mod wallet;
mod wasm;

pub use balance::Balance;
pub use network::Network;
pub use result::{CwuServiceError, Result};
pub use service::CwuService;
pub use wallet::Wallet;

pub trait CwuServiceTrait {
    fn create_wallet(&self) -> impl Future<Output = Result<Wallet>> + Send;
    fn check_balance(&self, address: &str) -> impl Future<Output = Result<Balance>> + Send;
}
