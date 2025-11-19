use crate::cli::service::ConsoleService;
use crate::cli::styles::message::Message;
use cwu_service::CwuServiceTrait;
use cwu_wallet::EncryptedWallet;
use dialoguer::console::Term;
use dialoguer::theme::Theme;

pub(super) enum OpenWallet {}

impl OpenWallet {
    pub(super) async fn apply(
        theme: &dyn Theme,
        term: &Term,
    ) -> Result<Option<EncryptedWallet>, anyhow::Error> {
        let name: String = dialoguer::Input::with_theme(theme)
            .with_prompt("Enter a wallet name or 'q' to quit")
            .interact_text_on(term)?;
        if name != "q" {
            let master_password: String = dialoguer::Password::with_theme(theme)
                .with_prompt("Enter a master password or 'q' to quit")
                .interact_on(term)?;
            if master_password != "q" {
                return match ConsoleService::new()
                    .open_wallet(name.as_str(), master_password)
                    .await
                {
                    Ok(wallet) => Ok(Some(wallet)),
                    Err(e) => {
                        Message::error(
                            format!("Can't open the wallet {}: error: {}", name, e).as_str(),
                        );
                        Ok(None)
                    }
                };
            }
        }
        Ok(None)
    }
}
