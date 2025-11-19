use std::fmt::{Display, Formatter};
use termimad::MadSkin;
use termimad::crossterm::style::Stylize;

pub struct StyledMnemonic {
    mnemonic: String,
}

impl StyledMnemonic {
    pub fn new(mnemonic: String) -> Self {
        StyledMnemonic { mnemonic }
    }
}

impl Display for StyledMnemonic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let skin = MadSkin::default();

        let header = "**-------------------- SECRET RECOVERY PHRASE -------------------**\n";
        let footer = format!("**{}**", "-".repeat(header.len())).red();

        let header = header.to_string().red();
        let info = "**Anyone who has this phrase controls all your funds**\n"
            .to_string()
            .red();
        let rules = "🚨 **Security Rules: Backup OFFLINE**\n";
        let rule1 = "- **NEVER** screenshot, photograph, email, or cloud-store this phrase\n";
        let rule2 = "- **NEVER** share this phrase with anyone\n";
        let backup = "✅ **Secure Backup Steps**\n".to_string().green();
        let step1 = "1. **Write** it down physically on paper or metal.\n";
        let step2 = "2. **Verify** the spelling and order of every word.\n";
        let step3 = "3. **Store copies** in separate, secure locations (e.g., safe, bank vault).\n";
        let mnemonic = format!("\n**{}**\n", self.mnemonic.to_uppercase()).green();

        let styled_message = format!(
            "{}{}{}{}{}{}{}{}{}{}{}",
            header, info, rules, rule1, rule2, backup, step1, step2, step3, mnemonic, footer
        );

        write!(f, "{}", skin.inline(&styled_message))
    }
}
