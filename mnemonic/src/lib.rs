use bip39::Mnemonic;

pub struct MasterSeed {
    mnemonic: Mnemonic,
}

impl MasterSeed {
    pub fn new() -> Self {
        let mnemonic = Mnemonic::generate(12).expect("Failed to generate mnemonic");
        Self { mnemonic }
    }

    pub fn to_string(&self) -> String {
        self.mnemonic.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let seed = MasterSeed::new();
        println!("{}", seed.to_string());
    }
}
