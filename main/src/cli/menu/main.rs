use crate::cli::menu::{check_balance::CheckBalance, new_wallet::NewWallet, wallet::WalletMenu};
use dialoguer::console::Term;
use dialoguer::theme::Theme;
use rand::prelude::IndexedRandom;

pub(crate) enum MainMenu {
    OpenWallet,
    CreateWallet,
    CheckBalance,
}

impl std::fmt::Display for MainMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MainMenu::OpenWallet => "Open existing wallet",
            MainMenu::CreateWallet => "Create a new wallet",
            MainMenu::CheckBalance => "Check balance",
        };
        f.write_str(s)
    }
}

impl MainMenu {
    pub async fn apply(theme: &dyn Theme, term: &Term) -> Result<(), anyhow::Error> {
        loop {
            let actions = [
                MainMenu::OpenWallet,
                MainMenu::CreateWallet,
                MainMenu::CheckBalance,
            ];

            let action = dialoguer::Select::with_theme(theme)
                .with_prompt("Pick an option (press 'q' to quit)")
                .items(&actions)
                .default(0)
                .interact_opt()?;

            let action = match action {
                Some(v) => v,
                None => {
                    println!("{}", get_random_farewell());
                    break; // Exit the main menu loop.
                }
            };

            match &actions[action] {
                MainMenu::OpenWallet => {
                    WalletMenu::apply(theme, term).await?;
                }
                MainMenu::CreateWallet => {
                    NewWallet::apply(theme, term).await?;
                }
                MainMenu::CheckBalance => {
                    CheckBalance::apply(theme, term).await?;
                }
            }
        }
        Ok(())
    }
}

fn get_random_farewell() -> &'static str {
    let farewells = [
        "Bye",
        "Adiós",
        "Tschüss",
        "Au revoir",
        "Ciao",
        "До побачення",
        "さようなら",
        "再见",
    ];

    // Create a thread-local random number generator
    let mut rng = rand::rng();

    farewells.choose(&mut rng).unwrap_or(&"До побачення")
}
