use crate::{Result, WalletError, language::Lang, network::get_tron_key_pair_from_mnemonic};
use bip39::Mnemonic;
use cwu_model::Network;
use cwu_security_utils::EncryptedPayload;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::{collections::HashMap, fs, path::Path, str::FromStr};
use zeroize::Zeroize;

#[derive(Debug)]
pub enum EncryptedWallet {
    Current(EncryptedWalletV1),
}

impl EncryptedWallet {
    pub fn create(word_count: i32, language: &str, wallet_name: &str) -> Result<String> {
        create(word_count, language, wallet_name)
    }

    pub fn open(wallet_name: &str, mut master_password: String) -> Result<EncryptedWallet> {
        let encrypted_payload = EncryptedPayload::from_file(wallet_file_name(wallet_name))?;
        let encrypted_wallet = cwu_security_utils::decrypt(&encrypted_payload, &master_password)?;
        master_password.zeroize();

        let version_container: VersionOnly = serde_json::from_str(&encrypted_wallet)?;
        match version_container.version {
            1 => {
                let v1: EncryptedWalletV1 = serde_json::from_str(&encrypted_wallet)?;
                Ok(EncryptedWallet::Current(v1))
            }
            _ => Err(WalletError::UnsupportedVersion(version_container.version)),
        }
    }

    pub fn backup(&self, mut master_password: String) -> Result<String> {
        let mnemonic = cwu_security_utils::decrypt(self.mnemonic(), &master_password)?;
        master_password.zeroize();
        Ok(mnemonic)
    }

    pub fn name(&self) -> &str {
        match self {
            EncryptedWallet::Current(w) => w.name.as_str(),
        }
    }

    pub(crate) fn mnemonic(&self) -> &EncryptedPayload {
        match self {
            EncryptedWallet::Current(w) => &w.mnemonic,
        }
    }
}

/// Lightweight Version Container .
#[derive(Debug, Deserialize)]
struct VersionOnly {
    version: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedWalletV1 {
    name: String,
    mnemonic: EncryptedPayload,
    passphrase: EncryptedPayload,
    key_pairs: HashMap<Network, EncryptedPayload>,
    version: u32,
}

impl EncryptedWalletV1 {
    fn new(
        wallet_name: &str,
        mnemonic: EncryptedPayload,
        passphrase: EncryptedPayload,
        key_pairs: HashMap<Network, EncryptedPayload>,
    ) -> Self {
        Self {
            name: wallet_name.to_string(),
            mnemonic,
            passphrase,
            key_pairs,
            version: 1,
        }
    }
}

const PASSWORD_LENGTH: usize = 8;

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
        wallet_name,
        mnemonic_encrypted,
        passphrase_encrypted,
        HashMap::from([(Network::Tron, tron_key_pair_encrypted)]),
    );
    let mut wallet_json_string = to_string_pretty(&wallet)?;
    let wallet_encrypted = cwu_security_utils::encrypt(&wallet_json_string, &master_password)?;

    // save wallet to file
    write_to_file(
        to_string_pretty(&wallet_encrypted)?,
        Path::new(wallet_file_name(wallet_name).as_str()),
    )?;
    wallet_json_string.zeroize();

    Ok(master_password)
}

const WALLET_FILE_NAME_SUFFIX: &str = ".cwu.json";

fn wallet_file_name(wallet_name: &str) -> String {
    format!("{wallet_name}{WALLET_FILE_NAME_SUFFIX}")
}

fn write_to_file(data: String, path: impl AsRef<Path>) -> Result<()> {
    fs::write(path, data)?;
    Ok(())
}
