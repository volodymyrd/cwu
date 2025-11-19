use crate::EncryptedWallet;

#[test]
fn test_open_wallet() {
    let master_password = "m(wLcdH7LR3w+5hb".to_string();
    let wallet = EncryptedWallet::open("src/tests/test1", master_password.clone()).unwrap();

    assert_eq!(wallet.name(), "test1");
    assert_eq!(
        wallet.backup(master_password.clone()).unwrap(),
        "comfort balcony breeze various join absent clock crime warrior detail kind push"
    );
}
