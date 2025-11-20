use crate::EncryptedWallet;
use cwu_model::Network;
use std::collections::HashMap;

#[test]
fn test_open_wallet() {
    let master_password = "TtWId4h$fm1y#(Nn".to_string();
    let wallet = EncryptedWallet::open("src/tests/test1", master_password.clone()).unwrap();

    assert_eq!(wallet.name(), "test1");
    assert_eq!(
        wallet.addresses(),
        &HashMap::from([(
            Network::Tron,
            "TMTpzDaQrCVsE1efSyCnsENcbBj2oUTjyX".to_string()
        )])
    );
    assert_eq!(
        wallet.backup(master_password.clone()).unwrap(),
        "fiber jazz upper cruel betray fence series suit habit ski crowd project"
    );
}
