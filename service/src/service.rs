use crate::wallet::Wallet;
use crate::wasm::Host;
use crate::{Balance, CwuServiceError, CwuServiceTrait, Network, Result};
use cwu_ether::Usdt;
use cwu_tron::Tron;

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
    async fn create_wallet(&self) -> Result<Wallet> {
        let mut host = Host::set_up()?;
        Ok(host.create_wallet()?)
    }

    async fn check_balance(&self, address: &str) -> Result<Balance> {
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
                    let tron = Tron::new().await?;
                    let trx_balance = tron.trx_balance(address).await;
                    if trx_balance.is_err() {
                        continue;
                    }
                    let usdt_balance = match tron.usdt_balance(address).await {
                        Ok(usdt) => usdt.to_string(),
                        Err(_) => "0 USDT".to_string(),
                    };
                    Ok(Balance::new(Network::Tron, usdt_balance))
                }
            };
        }
        Err(CwuServiceError::AddressNotFound)
    }
}
