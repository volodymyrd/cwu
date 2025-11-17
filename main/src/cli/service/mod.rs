use crate::cli::styles::progress::Progress;
use cwu_model::Balance;
use cwu_service::{CwuService, CwuServiceTrait, Result};

pub(crate) struct ConsoleService {
    internal: CwuService,
}

impl ConsoleService {
    pub fn new() -> Self {
        Self {
            internal: CwuService::new(),
        }
    }
}
impl CwuServiceTrait for ConsoleService {
    async fn create_wallet(
        &self,
        word_count: i32,
        language: &str,
        wallet_name: &str,
    ) -> Result<String> {
        let progress = Progress::with_spinner("Creating a new wallet...");
        let master_password = self
            .internal
            .create_wallet(word_count, language, wallet_name)
            .await;
        progress.finish();
        master_password
    }

    async fn check_balance(&self, address: &str) -> Result<Balance> {
        let progress = Progress::with_spinner("Checking balance...");
        let balance = self.internal.check_balance(address).await;
        progress.finish();
        balance
    }
}
