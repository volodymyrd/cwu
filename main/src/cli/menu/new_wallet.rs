use crate::cli::service::ConsoleService;
use cwu_service::CwuServiceTrait;
use dialoguer::console::Term;
use dialoguer::theme::Theme;

pub(super) struct NewWallet {}

impl NewWallet {
    pub async fn apply(theme: &dyn Theme, term: &Term) -> Result<(), anyhow::Error> {
        let wallet = ConsoleService::new().create_wallet().await?;
        println!("{}", wallet.mnemonic());

        Ok(())
    }
}
