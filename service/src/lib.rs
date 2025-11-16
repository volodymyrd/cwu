mod result;
mod service;
#[cfg(feature = "wasm")]
mod wasm;

use cwu_model::Balance;

pub use result::{CwuServiceError, Result};
pub use service::CwuService;

pub trait CwuServiceTrait {
    fn create_wallet(
        &self,
        word_count: i32,
        language: &str,
    ) -> impl Future<Output = Result<String>> + Send;

    fn check_balance(&self, address: &str) -> impl Future<Output = Result<Balance>> + Send;
}
