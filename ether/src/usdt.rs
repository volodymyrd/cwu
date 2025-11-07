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

#[derive(Debug)]
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
        Self::with_url_and_address(PUBLIC_RPC_URL, USDT_ADDRESS)
    }

    pub fn with_url_and_address(url: &str, address: Address) -> Result<Self> {
        let provider = ProviderBuilder::new().connect_http(url.parse()?);
        let contract = WETH9::new(address, provider.clone());
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

    #[test]
    fn test_new_usdt_fail_invalid_url() {
        let result = Usdt::with_url_and_address("invalid_url", USDT_ADDRESS);
        let err_msg = result.unwrap_err().to_string();
        assert_eq!(
            err_msg, "Invalid RPC URL provided: relative URL without a base",
            "Expected correct error message, but got '{}'",
            err_msg
        );
    }

    #[tokio::test]
    async fn test_name_fail_invalid_address() {
        let invalid_address = address!("0x0000000000000000000000000000000000000001");
        let usdt = Usdt::with_url_and_address(PUBLIC_RPC_URL, invalid_address).unwrap();
        let name = usdt.name().await;
        let err_msg = name.unwrap_err().to_string();
        assert!(
            err_msg.contains(
                "Contract error: contract call to `name` returned no data (\"0x\"); \
            the called address might not be a contract"
            ),
            "Expected correct error message, but got '{}'",
            err_msg
        );
    }

    #[tokio::test]
    async fn test_symbol_fail_invalid_address() {
        let invalid_address = address!("0x0000000000000000000000000000000000000001");
        let usdt = Usdt::with_url_and_address(PUBLIC_RPC_URL, invalid_address).unwrap();
        let symbol = usdt.symbol().await;
        let err_msg = symbol.unwrap_err().to_string();
        assert!(
            err_msg.contains(
                "Contract error: contract call to `symbol` returned no data (\"0x\"); \
            the called address might not be a contract"
            ),
            "Expected correct error message, but got '{}'",
            err_msg
        );
    }

    #[tokio::test]
    async fn test_decimals_fail_invalid_address() {
        let invalid_address = address!("0x0000000000000000000000000000000000000001");
        let usdt = Usdt::with_url_and_address(PUBLIC_RPC_URL, invalid_address).unwrap();
        let decimals = usdt.decimals().await;
        let err_msg = decimals.unwrap_err().to_string();
        assert!(
            err_msg.contains(
                "Contract error: contract call to `decimals` returned no data (\"0x\"); \
            the called address might not be a contract"
            ),
            "Expected correct error message, but got '{}'",
            err_msg
        );
    }

    #[tokio::test]
    async fn test_balance_fail_invalid_address() {
        let invalid_address = address!("0x0000000000000000000000000000000000000001");
        let wallet_address = address!("0x742d35Cc6634C0532925a3b844Bc454e4438f44e");
        let usdt = Usdt::with_url_and_address(PUBLIC_RPC_URL, invalid_address).unwrap();
        let balance = usdt.balance(wallet_address).await;
        let err_msg = balance.unwrap_err().to_string();
        assert!(
            err_msg.eq(
                "Contract error: contract call to `balanceOf` returned no data (\"0x\"); \
            the called address might not be a contract"
            ),
            "Expected correct error message, but got '{}'",
            err_msg
        );
    }

    #[test]
    fn test_usdt_balance_fail_invalid_decimals() {
        let usdt = Usdt::new().unwrap();
        let balance = U256::from(1u128);
        let decimals = 80; // format_units errors if decimals > 77
        let usdt_balance = usdt.usdt_balance(balance, decimals);
        let err_msg = usdt_balance.unwrap_err().to_string();
        assert_eq!(err_msg, "Units error: \"80\" is not a valid unit",);
    }
}
