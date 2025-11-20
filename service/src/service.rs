#[cfg(feature = "wasm")]
use crate::wasm::Host;
use crate::{CwuServiceError, CwuServiceTrait, Result};
use cwu_ether::Usdt;
use cwu_model::{Balance, Network};
use cwu_settings::CwuConfig;
use cwu_tron::Tron;
use cwu_wallet::EncryptedWallet;

pub struct CwuService {}

impl CwuService {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for CwuService {
    fn default() -> Self {
        Self::new()
    }
}

impl CwuServiceTrait for CwuService {
    async fn create_wallet(
        &self,
        word_count: i32,
        language: &str,
        wallet_name: &str,
    ) -> Result<String> {
        #[cfg(feature = "wasm")]
        {
            let mut host = Host::set_up()?;
            Ok(host.create_wallet(word_count, language)?)
        }
        #[cfg(not(feature = "wasm"))]
        {
            Ok(EncryptedWallet::create(word_count, language, wallet_name)?)
        }
    }

    async fn open_wallet(&self, name: &str, master_password: String) -> Result<EncryptedWallet> {
        Ok(EncryptedWallet::open(name, master_password)?)
    }

    async fn backup_wallet(
        &self,
        wallet: &EncryptedWallet,
        master_password: String,
    ) -> Result<String> {
        Ok(wallet.backup(master_password)?)
    }

    async fn check_balance(&self, address: &str, config: &CwuConfig) -> Result<Balance> {
        for network in Network::iter() {
            return match network {
                Network::Ethereum => {
                    let usdt = Usdt::new()?;
                    let balance = usdt.balance(address).await;
                    if balance.is_err() {
                        continue;
                    }
                    let balance = balance?;
                    let usdt_balance = if let Ok(usdt) = usdt.usdt_balance(balance, 6) {
                        usdt
                    } else {
                        "0 USDT".to_string()
                    };
                    Ok(Balance::new(Network::Ethereum, usdt_balance))
                }
                Network::Tron => {
                    let tron = Tron::new(config).await?;
                    let trx_balance = tron.trx_balance(address).await;
                    if trx_balance.is_err() {
                        eprintln!("Error: {}", trx_balance.err().unwrap());
                        continue;
                    }
                    let usdt_balance = match tron.usdt_balance(address).await {
                        Ok(usdt) => usdt.to_string(),
                        Err(err) => {
                            eprintln!("Error: {err}");
                            "0 USDT".to_string()
                        }
                    };
                    Ok(Balance::new(Network::Tron, usdt_balance))
                }
            };
        }
        Err(CwuServiceError::AddressNotFound)
    }
}
