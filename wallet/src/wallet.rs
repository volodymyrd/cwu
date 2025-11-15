use crate::{Result, language::Lang};
use bip39::Mnemonic;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::{
    fmt::{Display, Formatter},
    fs,
    path::Path,
    str::FromStr,
};

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    mnemonic: String,
    master_password: String,
    passphrase: String,
}

impl Wallet {
    pub fn create() -> Result<Wallet> {
        Ok(Wallet::new("".to_string(), "".to_string(), "".to_string()))
    }

    pub fn create_with(word_count: i32, language: &str) -> Result<Wallet> {
        let language = Lang::from_str(language)?.lang;
        let mnemonic = Mnemonic::generate_in(language, word_count as usize)?;
        let wallet_password = cwu_security_utils::generate_secure_password(8)?;
        let passphrase = cwu_security_utils::generate_secure_password(8)?;
        let master_password = format!("{}{}", wallet_password, passphrase);
        let wallet = Wallet::new(mnemonic.to_string(), master_password, passphrase);
        wallet.write_to_file(Path::new("wallet.cwu.json"))?;
        Ok(wallet)
    }

    fn new(mnemonic: String, master_password: String, passphrase: String) -> Self {
        Self {
            mnemonic,
            master_password,
            passphrase,
        }
    }

    pub fn write_to_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let json_string = to_string_pretty(self)?;

        let encrypted = cwu_security_utils::encrypt(&json_string, &self.master_password)?;
        let json_string = to_string_pretty(&encrypted)?;
        fs::write(path, json_string)?;

        Ok(())
    }

    pub fn read_from_file(path: impl AsRef<Path>) -> Result<Self> {
        let file_content = fs::read_to_string(path)?;

        let wallet: Self = serde_json::from_str(&file_content)?;

        Ok(wallet)
    }
}

impl Display for Wallet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Mnemonic: {}", self.mnemonic)?;
        writeln!(f, "Master Password: {}", self.master_password)?;
        write!(f, "Passphrase: {}", self.passphrase)
    }
}
