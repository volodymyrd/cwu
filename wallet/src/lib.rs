pub struct Wallet {
    mnemonic: String,
}
impl Wallet {
    pub fn new(mnemonic: String) -> Self {
        Self { mnemonic }
    }

    pub fn mnemonic(&self) -> &String {
        &self.mnemonic
    }
}
