mod result;
mod service;
#[cfg(feature = "wasm")]
mod wasm;

use cwu_model::Balance;
use cwu_wallet::EncryptedWallet;
pub use result::{CwuServiceError, Result};
pub use service::CwuService;

pub trait CwuServiceTrait {
    fn create_wallet(
        &self,
        word_count: i32,
        language: &str,
        wallet_name: &str,
    ) -> impl Future<Output = Result<String>> + Send;

    fn open_wallet(
        &self,
        name: &str,
        master_password: String,
    ) -> impl Future<Output = Result<EncryptedWallet>> + Send;

    fn backup_wallet(
        &self,
        wallet: &EncryptedWallet,
        master_password: String,
    ) -> impl Future<Output = Result<String>> + Send;

    fn check_balance(&self, address: &str) -> impl Future<Output = Result<Balance>> + Send;
}
