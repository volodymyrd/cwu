use crate::TronError;
use cwu_settings::CwuConfig;
use std::str::FromStr;
use tronic::{
    client::Client,
    contracts::{
        token::usdt::Usdt,
        trc20::{Trc20Calls, Trc20Contract},
    },
    domain::{address::TronAddress, trx::Trx},
    provider::grpc::GrpcProvider,
    signer::LocalSigner,
};

pub struct Tron<'a> {
    client: Client<GrpcProvider, LocalSigner>,
    // TRC20-based USDT smart contract address
    usdt_smart_contract_address: &'a str,
}

impl<'a> Tron<'a> {
    pub async fn new(config: &'a CwuConfig) -> crate::Result<Self> {
        let client = Client::builder()
            .provider(
                GrpcProvider::builder()
                    .connect(&config.tron.rpc_node)
                    .await?,
            )
            .signer(LocalSigner::rand())
            .build();
        let usdt_smart_contract_address = config
            .tron
            .usdt_smart_contract_address
            .as_ref()
            .ok_or(TronError::UsdtSmartContractAddressIsNotSet)?;
        Ok(Self {
            client,
            usdt_smart_contract_address,
        })
    }

    pub async fn trx_balance(&self, address: &str) -> crate::Result<Trx> {
        let tron_address = TronAddress::from_str(address)?;
        let balance = self
            .client
            .trx_balance()
            .address(tron_address)
            .get()
            .await?;
        Ok(balance)
    }

    pub async fn usdt_balance(&self, address: &str) -> crate::Result<Usdt> {
        let usdt_contract_tron_address =
            Trc20Contract::<Usdt>::new(TronAddress::from_str(self.usdt_smart_contract_address)?);
        let tron_address = TronAddress::from_str(address)?;
        let balance = self
            .client
            .trc20_balance_of()
            .contract(usdt_contract_tron_address)
            .owner(tron_address)
            .get()
            .await?;
        Ok(balance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_environment() -> CwuConfig {
        CwuConfig::test_new("src/tests/testing.toml")
    }

    #[tokio::test]
    async fn test_new_tron_client() {
        let config = setup_test_environment();
        let tron = Tron::new(&config).await;
        assert!(tron.is_ok());
    }

    #[tokio::test]
    async fn test_get_trx_balance_success() {
        let config = setup_test_environment();
        let tron = Tron::new(&config).await.unwrap();
        let balance = tron.trx_balance("TMTpzDaQrCVsE1efSyCnsENcbBj2oUTjyX").await;
        assert!(balance.is_ok());
    }

    #[tokio::test]
    async fn test_get_trx_balance_fail() {
        let config = setup_test_environment();
        let tron = Tron::new(&config).await.unwrap();
        let balance = tron.trx_balance("invalid-address").await;
        assert!(balance.is_err());
        let error = balance.err().unwrap();
        assert_eq!(error.to_string(), "Error: bad address");
    }

    #[tokio::test]
    async fn test_get_usdt_balance_success() {
        let config = setup_test_environment();
        let tron = Tron::new(&config).await.unwrap();
        let balance = tron
            .usdt_balance("TQnjctUA8Lue5ggrY39BouA3r6CLgxfPVP")
            .await;
        assert!(balance.is_ok());
    }

    #[tokio::test]
    async fn test_get_usdt_balance_fail() {
        let config = setup_test_environment();
        let tron = Tron::new(&config).await.unwrap();
        let balance = tron.usdt_balance("invalid-address").await;
        assert!(balance.is_err());
        let error = balance.err().unwrap();
        assert_eq!(error.to_string(), "Error: bad address");
    }
}
