use bip39::Mnemonic;
use std::fmt::Display;

pub struct MasterSeed {
    mnemonic: Mnemonic,
}

impl MasterSeed {
    pub fn new() -> Self {
        let mnemonic = Mnemonic::generate(12).expect("Failed to generate mnemonic");
        Self { mnemonic }
    }
}

impl Default for MasterSeed {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for MasterSeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.mnemonic)
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
