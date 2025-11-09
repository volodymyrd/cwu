use crate::cli::menu::check_balance::CheckBalance;
use dialoguer::console::Term;
use dialoguer::theme::Theme;

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
                .with_prompt("Pick an option")
                .items(&actions)
                .default(0)
                .interact_opt()?;

            let action = match action {
                Some(v) => v,
                None => {
                    // User pressed 'Esc' or 'q' on the main menu, treat as "Quit".
                    println!("\nExiting application (via Esc/q).");
                    break; // Exit the main menu loop.
                }
            };

            match &actions[action] {
                MainMenu::OpenWallet => {
                    println!("Open wallet");
                }
                MainMenu::CreateWallet => {
                    println!("Create wallet");
                }
                MainMenu::CheckBalance => {
                    CheckBalance::apply(theme, term).await?;
                }
            }
        }
        Ok(())
    }
}
