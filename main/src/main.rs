use crate::cli::menu::main::MainMenu;
use dialoguer::{console::Term, theme::ColorfulTheme};

mod cli;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let theme = ColorfulTheme::default();
    let term = Term::buffered_stderr();
    let config = cwu_settings::CwuConfig::new()?;
    MainMenu::apply(&theme, &term, &config).await?;
    Ok(())
}
