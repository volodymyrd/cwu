use crate::weth9::{WETH9, WETH9::WETH9Instance};
use crate::{PUBLIC_RPC_URL, Result};
use alloy::{
    primitives::{
        Address, U256, address,
        utils::{format_ether, format_units},
    },
    providers::{
        Identity, ProviderBuilder, RootProvider,
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
    },
};

// USDT Contract Address on Ethereum Mainnet
const USDT_ADDRESS: Address = address!("0xdAC17F958D2ee523a2206206994597C13D831ec7");

struct Usdt {
    contract: WETH9Instance<
        FillProvider<
            JoinFill<
                Identity,
                JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
            >,
            RootProvider,
        >,
    >,
}

impl Usdt {
    pub fn new() -> Result<Self> {
        let provider = ProviderBuilder::new().connect_http(PUBLIC_RPC_URL.parse()?);
        let contract = WETH9::new(USDT_ADDRESS, provider.clone());
        Ok(Self { contract })
    }

    pub async fn name(&self) -> Result<String> {
        let result = self.contract.name().call().await?;
        Ok(result)
    }

    pub async fn symbol(&self) -> Result<String> {
        let result = self.contract.symbol().call().await?;
        Ok(result)
    }

    pub async fn decimals(&self) -> Result<u8> {
        let result = self.contract.decimals().call().await?;
        Ok(result)
    }

    pub async fn balance(&self, wallet_address: Address) -> Result<U256> {
        let result = self.contract.balanceOf(wallet_address).call().await?;
        Ok(result)
    }

    pub fn ether_balance(&self, balance: U256) -> String {
        format_ether(balance)
    }

    pub fn usdt_balance(&self, balance: U256, decimals: u8) -> Result<String> {
        Ok(format_units(balance, decimals)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloy::primitives::address;

    #[test]
    fn test_new_usdt_success() {
        let usdt = Usdt::new();
        assert!(usdt.is_ok());
    }

    #[tokio::test]
    async fn test_name_success() {
        let usdt = Usdt::new().unwrap();
        let name = usdt.name().await;
        assert!(name.is_ok());
        assert_eq!(name.unwrap(), "Tether USD");
    }

    #[tokio::test]
    async fn test_symbol_success() {
        let usdt = Usdt::new().unwrap();
        let symbol = usdt.symbol().await;
        assert!(symbol.is_ok());
        assert_eq!(symbol.unwrap(), "USDT");
    }

    #[tokio::test]
    async fn test_decimals_success() {
        let usdt = Usdt::new().unwrap();
        let decimals = usdt.decimals().await;
        assert!(decimals.is_ok());
        assert_eq!(decimals.unwrap(), 6);
    }

    #[tokio::test]
    async fn test_balance_success() {
        let usdt = Usdt::new().unwrap();
        let wallet_address = address!("0x742d35Cc6634C0532925a3b844Bc454e4438f44e"); // Kraken exchange wallet
        let balance = usdt.balance(wallet_address).await;
        assert!(balance.is_ok());
    }

    #[test]
    fn test_ether_balance_success() {
        let usdt = Usdt::new().unwrap();
        let balance = U256::from(1000000000000000000u128);
        let ether_balance = usdt.ether_balance(balance);
        assert_eq!(ether_balance, "1.000000000000000000");
    }

    #[test]
    fn test_usdt_balance_success() {
        let usdt = Usdt::new().unwrap();
        let balance = U256::from(1000000u128);
        let decimals = 6;
        let usdt_balance = usdt.usdt_balance(balance, decimals);
        assert!(usdt_balance.is_ok());
        assert_eq!(usdt_balance.unwrap(), "1.000000");
    }

    #[tokio::test]
    async fn test_balance_of_zero_address_is_not_zero() {
        let usdt = Usdt::new().unwrap();
        let wallet_address = address!("0x0000000000000000000000000000000000000000");
        let balance = usdt.balance(wallet_address).await;
        assert!(balance.is_ok());
        assert!(balance.unwrap() > U256::from(0));
    }
}
