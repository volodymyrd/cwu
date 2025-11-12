use crate::cli::styles::progress::Progress;
use cwu_service::{Balance, CwuService, CwuServiceTrait, Result, Wallet};

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
    async fn create_wallet(&self, word_count: i32, language: &str) -> Result<Wallet> {
        let progress = Progress::with_spinner("Creating a new wallet...");
        let wallet = self.internal.create_wallet(word_count, language).await;
        progress.finish();
        wallet
    }

    async fn check_balance(&self, address: &str) -> Result<Balance> {
        let progress = Progress::with_spinner("Checking balance...");
        let balance = self.internal.check_balance(address).await;
        progress.finish();
        balance
    }
}
