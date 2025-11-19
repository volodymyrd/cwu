use crate::cli::service::ConsoleService;
use crate::cli::styles::mnemonic::StyledMnemonic;
use cwu_service::CwuServiceTrait;
use cwu_wallet::EncryptedWallet;
use dialoguer::console::Term;
use dialoguer::theme::Theme;

pub(super) enum BackupWallet {}

impl BackupWallet {
    pub(super) async fn apply(
        theme: &dyn Theme,
        term: &Term,
        wallet: &EncryptedWallet,
    ) -> Result<(), anyhow::Error> {
        let master_password: String = dialoguer::Input::with_theme(theme)
            .with_prompt("Enter a master password or 'q' to quit")
            .interact_text_on(term)?;
        if master_password != "q" {
            let mnemonic = ConsoleService::new()
                .backup_wallet(wallet, master_password)
                .await?;
            println!("{}", StyledMnemonic::new(mnemonic));
        }
        Ok(())
    }
}
