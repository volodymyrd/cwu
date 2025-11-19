mod backup;
mod open_wallet;

use crate::cli::menu::wallet::backup::BackupWallet;
use crate::cli::menu::wallet::open_wallet::OpenWallet;
use dialoguer::console::Term;
use dialoguer::theme::Theme;

pub(super) enum WalletMenu {
    NewTransaction,
    History,
    Backup,
    Exit,
}

impl std::fmt::Display for WalletMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            WalletMenu::NewTransaction => "Start a new transaction",
            WalletMenu::History => "Show transaction history",
            WalletMenu::Backup => "Make a backup",
            WalletMenu::Exit => "Exit to the main menu",
        };
        f.write_str(s)
    }
}

impl WalletMenu {
    pub(super) async fn apply(theme: &dyn Theme, term: &Term) -> Result<(), anyhow::Error> {
        let wallet = OpenWallet::apply(theme, term).await?;
        if wallet.is_none() {
            return Ok(());
        }
        let wallet = wallet.unwrap();
        loop {
            let actions = [
                WalletMenu::NewTransaction,
                WalletMenu::History,
                WalletMenu::Backup,
                WalletMenu::Exit,
            ];

            let action = dialoguer::Select::with_theme(theme)
                .with_prompt(format!(
                    "Wallet '{}': Pick an option (press 'q' to quit)",
                    &wallet.name()
                ))
                .items(&actions)
                .default(0)
                .interact_opt()?;

            let action = match action {
                Some(v) => v,
                None => {
                    break;
                }
            };

            match &actions[action] {
                WalletMenu::NewTransaction => {
                    println!("New transaction");
                }
                WalletMenu::History => {
                    println!("History");
                }
                WalletMenu::Backup => {
                    BackupWallet::apply(theme, term, &wallet).await?;
                }
                WalletMenu::Exit => break,
            }
        }
        Ok(())
    }
}
