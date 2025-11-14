use std::fmt::{Display, Formatter};
use termimad::MadSkin;
use termimad::crossterm::style::Stylize;
use cwu_model::Balance;

pub struct StyledBalance<'a> {
    balance: &'a Balance,
}

impl<'a> StyledBalance<'a> {
    pub fn new(balance: &'a Balance) -> Self {
        StyledBalance { balance }
    }
}

impl<'a> Display for StyledBalance<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let skin = MadSkin::default();

        let header = "--- CURRENT BALANCE ---";
        let footer = format!("**{}**", "-".repeat(header.len()));

        let styled_message = format!(
            "**{}**\n**Network:** {}\n**USDT:** {}\n{}",
            header.green(),
            self.balance.network(),
            self.balance.usdt(),
            footer.green()
        );

        write!(f, "{}", skin.inline(&styled_message))
    }
}
