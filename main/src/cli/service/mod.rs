use crate::cli::styles::progress::Progress;
use cwu_model::Balance;
use cwu_service::{CwuService, CwuServiceTrait, Result};
use cwu_settings::CwuConfig;
use cwu_wallet::EncryptedWallet;

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

    async fn open_wallet(&self, name: &str, master_password: String) -> Result<EncryptedWallet> {
        let progress = Progress::with_spinner(format!("Opening wallet '{}'...", name).as_str());
        let wallet = self.internal.open_wallet(name, master_password).await;
        progress.finish();
        wallet
    }

    async fn backup_wallet(
        &self,
        wallet: &EncryptedWallet,
        master_password: String,
    ) -> Result<String> {
        let progress =
            Progress::with_spinner(format!("Backup wallet '{}'...", wallet.name()).as_str());
        let mnemonic = self.internal.backup_wallet(wallet, master_password).await;
        progress.finish();
        mnemonic
    }

    async fn check_balance(&self, address: &str, config: &CwuConfig) -> Result<Balance> {
        let progress = Progress::with_spinner("Checking balance...");
        let balance = self.internal.check_balance(address, config).await;
        progress.finish();
        balance
    }
}
