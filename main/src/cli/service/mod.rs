use crate::cli::styles::progress::Progress;
use cwu_service::{Balance, CwuService, CwuServiceTrait, Result};

pub(crate) struct ConsoleService {}

impl ConsoleService {
    pub fn new() -> Self {
        Self {}
    }
}
impl CwuServiceTrait for ConsoleService {
    async fn check_balance(&self, address: &str) -> Result<Balance> {
        let progress = Progress::with_spinner("Checking balance...");
        let balance = CwuService::new().check_balance(address).await;
        progress.finish();
        balance
    }
}
