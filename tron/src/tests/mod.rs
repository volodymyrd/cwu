use std::fs::File;
use std::io::Read;
// https://developers.tron.network/docs/getting-testnet-tokens-on-tron
// https://nile.tronscan.org
// https://shasta.tronscan.org/
use cwu_settings::CwuConfig;
use std::str::FromStr;
use tronic::client::Client;
use tronic::client::pending::{AutoSigning, ManualSigning, PendingTransaction};
use tronic::contracts::trc20::Trc20Calls;
use tronic::domain::Message;
use tronic::domain::address::TronAddress;
use tronic::domain::contract::{Contract, ContractType, ResourceCode, TransferContract};
use tronic::domain::transaction::Transaction;
use tronic::domain::trx::Trx;
use tronic::provider::TronProvider;
use tronic::provider::grpc::GrpcProvider;
use tronic::provider::mock::MockProvider;
use tronic::signer::LocalSigner;
const SIGNED_TRANSACTION_FILE: &str = "signed_transaction.bin";
const PRIVATE_KEY_FILE: &str = "private_key.hex";

fn setup_test_environment() -> CwuConfig {
    CwuConfig::test_new("src/tests/testing.toml")
}

#[test]
fn test() {
    assert!(TronAddress::from_str("TMTpzDaQrCVsE1efSyCnsENcbBj2oUTjyX").is_ok());
    assert!(TronAddress::from_str("TMTpzDaQrCVsE1efSyCnsENcbBj2oUThyX").is_err())
}

#[tokio::test]
#[ignore]
async fn standard_transaction() -> anyhow::Result<()> {
    let config = setup_test_environment();

    let signer = LocalSigner::from_bytes(&hex::decode(
        "b51cde5cc2875fb879d49dda436dbeda1c26f4941df9bf47849dcf73841b6b9e",
    )?)?;
    let sender_address = signer.address();
    println!("Sender address: {sender_address}");

    let client = Client::builder()
        .provider(
            GrpcProvider::builder()
                .connect(config.tron.rpc_node)
                .await?,
        )
        .signer(signer)
        .build();

    let recipient = TronAddress::from_str("TQKzP25THvRXFWigHWaiNNbgsbXZ3D9Gj4")?;
    let amount: Trx = "0.01".trim().parse::<f64>()?.into();
    println!("Sending {amount} to {recipient}...",);

    // Send transaction
    let txid = client
        .send_trx()
        .to(recipient)
        .amount(amount)
        .can_spend_trx_for_fee(true)
        .build::<AutoSigning>() // Uses automatic signing strategy
        .await?
        .broadcast(&())
        .await?;

    println!("Transaction sent successfully!");
    println!("Transaction hash: {txid:?}",);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn create_and_save_manual_signed_transaction() -> anyhow::Result<()> {
    let config = setup_test_environment();

    let provider = GrpcProvider::builder()
        .connect(config.tron.rpc_node)
        .await?;
    let latest_block = provider.get_now_block().await?;
    let client = Client::builder()
        .provider(provider)
        .signer(LocalSigner::rand())
        .build();

    let owner_address = TronAddress::from_str("TMTpzDaQrCVsE1efSyCnsENcbBj2oUTjyX")?;
    let to_address = TronAddress::from_str("TQKzP25THvRXFWigHWaiNNbgsbXZ3D9Gj4")?;
    let amount: Trx = "0.01".trim().parse::<f64>()?.into();

    let transfer_contract = TransferContract {
        owner_address,
        to_address,
        amount,
    };
    let contract = Contract {
        contract_type: ContractType::TransferContract(transfer_contract),
        ..Default::default()
    };
    let transaction = Transaction::new(
        contract,
        &latest_block,
        Message::default(), // Or add a memo if needed
    );
    let mut pd: PendingTransaction<GrpcProvider, LocalSigner, ManualSigning> =
        PendingTransaction::new(&client, transaction, owner_address, Trx::ZERO, true).await?;

    let ser = pd.serialize();
    println!("PendingTransaction was created successfully!");

    //
    let signer = LocalSigner::from_bytes(&hex::decode(
        "b51cde5cc2875fb879d49dda436dbeda1c26f4941df9bf47849dcf73841b6b9e",
    )?)?;
    let offline_client = Client::builder()
        .provider(MockProvider::new().await)
        .signer(LocalSigner::rand())
        .build();
    let mut pd = PendingTransaction::try_deserialize(&offline_client, &ser)?;
    pd.sign(&signer, &()).await?;
    let ser = pd.serialize();
    println!("PendingTransaction was signed successfully!");

    //
    println!("Try to send transaction...");
    let pd = PendingTransaction::try_deserialize(&client, &ser)?;
    let txid = pd.broadcast().await?;
    println!("Transaction sent successfully!");
    //println!("Transaction hash: {txid:?}",);
    Ok(())
}

#[tokio::test]
#[ignore]
async fn load_and_broadcast_transaction() -> anyhow::Result<()> {
    let config = setup_test_environment();

    // 1. Read the serialized signed transaction
    let mut file = File::open(SIGNED_TRANSACTION_FILE)?;
    let mut serialized_tx = Vec::new();
    file.read_to_end(&mut serialized_tx)?;

    // 2. Create a dummy client to deserialize the transaction.
    // The actual signer is not needed for deserialization, only for broadcasting.
    // We need a client reference for PendingTransaction::try_deserialize.
    let provider = GrpcProvider::builder()
        .connect(config.tron.rpc_node)
        .await?;
    let dummy_signer = LocalSigner::rand(); // Dummy signer, as the transaction is already signed
    let client = Client::builder()
        .provider(provider)
        .signer(dummy_signer)
        .build();

    // 3. Deserialize the signed transaction
    let pending_transaction =
        tronic::client::pending::PendingTransaction::<_, _, ManualSigning>::try_deserialize(
            &client,
            &serialized_tx,
        )?;

    // 4. Broadcast the already signed transaction
    let txid = pending_transaction.broadcast().await?;

    println!("Transaction sent successfully!");
    println!("Transaction hash: {txid:?}");

    Ok(())
}
