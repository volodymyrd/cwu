use crate::cli::menu::main::MainMenu;
use dialoguer::{console::Term, theme::ColorfulTheme};

mod cli;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let theme = ColorfulTheme::default();
    let term = Term::buffered_stderr();
    MainMenu::apply(&theme, &term).await?;
    Ok(())
}
