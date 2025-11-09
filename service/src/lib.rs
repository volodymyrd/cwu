mod balance;
mod network;
mod result;
mod service;

pub use balance::Balance;
pub use network::Network;
pub use result::{CwuServiceError, Result};
pub use service::CwuService;

pub trait CwuServiceTrait {
    fn check_balance(&self, address: &str) -> impl Future<Output = Result<Balance>> + Send;
}
