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

const PUBLIC_GRPC_URL: &str = "http://grpc.trongrid.io:50051";
// TRC20-based USDT smart contract address
const USDT_SMART_CONTRACT_ADDRESS: &str = "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t";

pub struct Tron {
    client: Client<GrpcProvider, LocalSigner>,
}

impl Tron {
    pub async fn new() -> crate::Result<Self> {
        let client = Client::builder()
            .provider(GrpcProvider::builder().connect(PUBLIC_GRPC_URL).await?)
            .signer(LocalSigner::rand())
            .build();
        Ok(Self { client })
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
            Trc20Contract::<Usdt>::new(TronAddress::from_str(USDT_SMART_CONTRACT_ADDRESS)?);
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

    #[tokio::test]
    async fn test_new_tron_client() {
        let tron = Tron::new().await;
        assert!(tron.is_ok());
    }

    #[tokio::test]
    async fn test_get_trx_balance_success() {
        let tron = Tron::new().await.unwrap();
        let balance = tron.trx_balance("TNPeeaaFB7K9cmo4uQpcU32zGK8G1NYqeL").await;
        assert!(balance.is_ok());
    }

    #[tokio::test]
    async fn test_get_trx_balance_fail() {
        let tron = Tron::new().await.unwrap();
        let balance = tron.trx_balance("invalid-address").await;
        assert!(balance.is_err());
        let error = balance.err().unwrap();
        assert_eq!(error.to_string(), "Error: bad address");
    }

    #[tokio::test]
    async fn test_get_usdt_balance_success() {
        let tron = Tron::new().await.unwrap();
        let balance = tron
            .usdt_balance("TNPeeaaFB7K9cmo4uQpcU32zGK8G1NYqeL")
            .await;
        assert!(balance.is_ok());
    }

    #[tokio::test]
    async fn test_get_usdt_balance_fail() {
        let tron = Tron::new().await.unwrap();
        let balance = tron.usdt_balance("invalid-address").await;
        assert!(balance.is_err());
        let error = balance.err().unwrap();
        assert_eq!(error.to_string(), "Error: bad address");
    }
}
