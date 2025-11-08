use std::str::FromStr;
use tronic::client::Client;
use tronic::contracts::token::usdt::Usdt;
use tronic::contracts::trc20::{Trc20Calls, Trc20Contract};
use tronic::domain::address::TronAddress;
use tronic::domain::trx::Trx;
use tronic::provider::grpc::GrpcProvider;
use tronic::signer::LocalSigner;

const PUBLIC_GRPC_URL: &str = "http://grpc.trongrid.io:50051";
// TRC20-based USDT smart contract address
const USDT_SMART_CONTRACT_ADDRESS: &str = "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t";

#[derive(thiserror::Error, Debug)]
pub enum TronError {
    #[error("Tronic error: {0}")]
    TronicError(#[from] tronic::error::Error),

    #[error("Error: {0}")]
    Error(#[from] anyhow::Error),

    #[error("An error occurred: {0}")]
    Err(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl From<Box<dyn std::error::Error + Send + Sync + 'static>> for TronError {
    fn from(err: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        TronError::Err(err)
    }
}

type Result<T> = std::result::Result<T, TronError>;

pub struct Tron {
    client: Client<GrpcProvider, LocalSigner>,
}

impl Tron {
    pub async fn new() -> Result<Self> {
        let client = Client::builder()
            .provider(GrpcProvider::builder().connect(PUBLIC_GRPC_URL).await?)
            .signer(LocalSigner::rand())
            .build();
        Ok(Self { client })
    }

    pub async fn get_trx_balance(&self, address: &str) -> Result<Trx> {
        let tron_address = TronAddress::from_str(address)?;
        let balance = self
            .client
            .trx_balance()
            .address(tron_address)
            .get()
            .await?;
        Ok(balance)
    }

    pub async fn get_usdt_balance(&self, address: &str) -> Result<Usdt> {
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
