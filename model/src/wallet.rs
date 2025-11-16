use crate::Network;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct Wallet {
    mnemonic: String,
    master_password: String,
    addresses: HashMap<Network, String>,
}

impl Wallet {
    pub fn new(mnemonic: &str, master_password: &str, addresses: HashMap<Network, String>) -> Self {
        Self {
            mnemonic: mnemonic.to_string(),
            master_password: master_password.to_string(),
            addresses,
        }
    }
}

impl Display for Wallet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Mnemonic: {}", self.mnemonic)?;
        writeln!(f, "Master Password: {}", self.master_password)?;
        writeln!(f, "Addresses:")?;
        for (network, address) in &self.addresses {
            writeln!(f, "{}: {}", network, address)?;
        }
        Ok(())
    }
}
