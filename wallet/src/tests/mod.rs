use crate::EncryptedWallet;
use crate::key_pair::KeyPair;
use cwu_model::Network;
use std::collections::HashMap;

#[test]
fn test_open_wallet1() {
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

    let tron_key_pair = wallet
        .key_pair(master_password.clone(), Network::Tron)
        .unwrap();

    assert_eq!(
        tron_key_pair,
        KeyPair::new(
            "b51cde5cc2875fb879d49dda436dbeda1c26f4941df9bf47849dcf73841b6b9e".to_string(),
            "TMTpzDaQrCVsE1efSyCnsENcbBj2oUTjyX".to_string()
        )
    )
}

#[test]
fn test_open_wallet2() {
    let master_password = "CQLav?I4e]oLm7;J".to_string();
    let wallet = EncryptedWallet::open("src/tests/test2", master_password.clone()).unwrap();

    assert_eq!(wallet.name(), "test2");
    assert_eq!(
        wallet.addresses(),
        &HashMap::from([(
            Network::Tron,
            "TQKzP25THvRXFWigHWaiNNbgsbXZ3D9Gj4".to_string()
        )])
    );
    assert_eq!(
        wallet.backup(master_password.clone()).unwrap(),
        "option catalog claim zero waste nut congress wasp student frown desert route"
    );

    let tron_key_pair = wallet
        .key_pair(master_password.clone(), Network::Tron)
        .unwrap();

    assert_eq!(
        tron_key_pair,
        KeyPair::new(
            "f4517a9529f827f2c946e57f6f3c0f19a7ac76f981a110b682e2107b70162b4b".to_string(),
            "TQKzP25THvRXFWigHWaiNNbgsbXZ3D9Gj4".to_string()
        )
    )
}
