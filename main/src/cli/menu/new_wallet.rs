use crate::cli::service::ConsoleService;
use colored::Colorize;
use cwu_service::CwuServiceTrait;
use dialoguer::console::Term;
use dialoguer::theme::Theme;
use termimad::MadSkin;

pub(super) struct NewWallet {}

impl NewWallet {
    pub async fn apply(theme: &dyn Theme, term: &Term) -> Result<(), anyhow::Error> {
        let lang = Language::apply(theme, term)?;
        if lang.is_none() {
            return Ok(());
        }
        let word_count = WordCount::apply(theme, term)?;
        if word_count.is_none() {
            return Ok(());
        }
        let wallet_name: String = dialoguer::Input::with_theme(theme)
            .with_prompt("Come up with a wallet name or 'q' to quit")
            .interact_text_on(term)?;
        if wallet_name.is_empty() || wallet_name == "q" {
            return Ok(());
        }
        let master_password = ConsoleService::new()
            .create_wallet(
                i32::from(word_count.unwrap()),
                &lang.unwrap().to_string(),
                wallet_name.as_str(),
            )
            .await?;

        let skin = MadSkin::default();
        let styled_message = format!(
            "**Master Password (between < >):** <**{}**>",
            master_password.red()
        );
        println!("{}", skin.inline(&styled_message));
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Language {
    English,
    Spanish,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Language::English => "English",
            Language::Spanish => "Spanish",
        };
        f.write_str(s)
    }
}

impl Language {
    pub fn apply(theme: &dyn Theme, _: &Term) -> Result<Option<Language>, anyhow::Error> {
        let actions = [Language::English, Language::Spanish];

        let action = dialoguer::Select::with_theme(theme)
            .with_prompt("Pick a language (press 'q' to back)")
            .items(actions)
            .default(0)
            .interact_opt()?;

        let action = match action {
            Some(v) => v,
            None => {
                return Ok(None);
            }
        };

        Ok(Some(actions[action]))
    }
}

#[derive(Clone, Copy)]
pub(crate) enum WordCount {
    Twelve,
    TwentyFour,
}

impl From<WordCount> for i32 {
    fn from(count: WordCount) -> i32 {
        match count {
            WordCount::Twelve => 12,
            WordCount::TwentyFour => 24,
        }
    }
}

impl std::fmt::Display for WordCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            WordCount::Twelve => "12",
            WordCount::TwentyFour => "24",
        };
        f.write_str(s)
    }
}

impl WordCount {
    pub fn apply(theme: &dyn Theme, _: &Term) -> Result<Option<WordCount>, anyhow::Error> {
        let actions = [WordCount::Twelve, WordCount::TwentyFour];

        let action = dialoguer::Select::with_theme(theme)
            .with_prompt("Pick a number of words (press 'q' to back)")
            .items(actions)
            .default(0)
            .interact_opt()?;

        let action = match action {
            Some(v) => v,
            None => {
                return Ok(None);
            }
        };

        Ok(Some(actions[action]))
    }
}
