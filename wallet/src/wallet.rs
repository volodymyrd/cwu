use crate::{Result, language::Lang, network::get_tron_key_pair_from_mnemonic};
use bip39::Mnemonic;
use cwu_model::Network;
use cwu_security_utils::EncryptedPayload;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::{collections::HashMap, fs, path::Path, str::FromStr};
use zeroize::Zeroize;

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedWalletV1 {
    mnemonic: EncryptedPayload,
    passphrase: EncryptedPayload,
    key_pairs: HashMap<Network, EncryptedPayload>,
}

impl EncryptedWalletV1 {
    pub fn create(word_count: i32, language: &str, wallet_name: &str) -> Result<String> {
        InternalWallet::create(word_count, language, wallet_name)
    }

    pub(crate) fn new(
        mnemonic: EncryptedPayload,
        passphrase: EncryptedPayload,
        key_pairs: HashMap<Network, EncryptedPayload>,
    ) -> Self {
        Self {
            mnemonic,
            passphrase,
            key_pairs,
        }
    }

    pub fn write_to_file(&self, data: String, path: impl AsRef<Path>) -> Result<()> {
        fs::write(path, data)?;
        Ok(())
    }

    pub fn read_from_file(path: impl AsRef<Path>) -> Result<Self> {
        let file_content = fs::read_to_string(path)?;
        let wallet: Self = serde_json::from_str(&file_content)?;
        Ok(wallet)
    }
}

struct InternalWallet {}

const PASSWORD_LENGTH: usize = 8;

impl InternalWallet {
    fn create(word_count: i32, language: &str, wallet_name: &str) -> Result<String> {
        let language = Lang::from_str(language)?.lang;
        let mnemonic = Mnemonic::generate_in(language, word_count as usize)?;

        let mut wallet_password = cwu_security_utils::generate_secure_password(PASSWORD_LENGTH)?;
        let mut passphrase = cwu_security_utils::generate_secure_password(PASSWORD_LENGTH)?;
        let master_password = format!("{wallet_password}{passphrase}");
        wallet_password.zeroize();

        let mut tron_key_pair = get_tron_key_pair_from_mnemonic(&mnemonic, &passphrase)?;
        let mut tron_key_pair_str = tron_key_pair.serialize()?;
        let tron_key_pair_encrypted =
            cwu_security_utils::encrypt(&tron_key_pair_str, &master_password)?;
        tron_key_pair.zeroize();
        tron_key_pair_str.zeroize();

        let passphrase_encrypted = cwu_security_utils::encrypt(&passphrase, &master_password)?;
        passphrase.zeroize();

        let mut mnemonic_str = mnemonic.to_string();
        let mnemonic_encrypted = cwu_security_utils::encrypt(&mnemonic_str, &master_password)?;
        mnemonic_str.zeroize();
        let wallet = EncryptedWalletV1::new(
            mnemonic_encrypted,
            passphrase_encrypted,
            HashMap::from([(Network::Tron, tron_key_pair_encrypted)]),
        );
        let mut wallet_json_string = to_string_pretty(&wallet)?;
        let wallet_encrypted = cwu_security_utils::encrypt(&wallet_json_string, &master_password)?;

        // save wallet to file
        wallet.write_to_file(
            to_string_pretty(&wallet_encrypted)?,
            Path::new(format!("{wallet_name}.cwu.json").as_str()),
        )?;
        wallet_json_string.zeroize();

        Ok(master_password)
    }
}
