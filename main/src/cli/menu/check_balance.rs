use crate::cli::service::ConsoleService;
use crate::cli::styles::{balance::StyledBalance, message::Message};
use cwu_service::CwuServiceTrait;
use dialoguer::{console::Term, theme::Theme};

pub(super) enum CheckBalance {}

impl CheckBalance {
    pub async fn apply(theme: &dyn Theme, term: &Term) -> Result<(), anyhow::Error> {
        let address: String = dialoguer::Input::with_theme(theme)
            .with_prompt("Enter an address or 'q' to quit")
            .interact_text_on(term)?;
        if address != "q" {
            match ConsoleService::new().check_balance(&address).await {
                Ok(balance) => println!("{}", StyledBalance::new(&balance)),
                Err(_) => Message::error("Address not found!"),
            };
        }

        Ok(())
    }
}
