use crate::cli::service::ConsoleService;
use crate::cli::styles::{balance::StyledBalance, message::Message};
use cwu_service::CwuServiceTrait;
use cwu_settings::CwuConfig;
use dialoguer::{console::Term, theme::Theme};

pub(super) enum CheckBalance {}

impl CheckBalance {
    pub(super) async fn apply(
        theme: &dyn Theme,
        term: &Term,
        config: &CwuConfig,
    ) -> Result<(), anyhow::Error> {
        let address: String = dialoguer::Input::with_theme(theme)
            .with_prompt("Enter an address or 'q' to quit")
            .interact_text_on(term)?;
        if address != "q" {
            match ConsoleService::new()
                .check_balance(address.as_str(), config)
                .await
            {
                Ok(balance) => println!("{}", StyledBalance::new(&balance)),
                Err(_) => Message::error("Address not found!"),
            };
        }

        Ok(())
    }
}
