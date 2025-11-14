use crate::network::Network;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Balance {
    network: Network,
    usdt: String,
}

impl Display for Balance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Network: {}", self.network)?;
        write!(f, "USDT Balance: {}", self.usdt)
    }
}

impl Balance {
    pub fn new(network: Network, usdt: String) -> Self {
        Self { network, usdt }
    }

    pub fn network(&self) -> &Network {
        &self.network
    }

    pub fn usdt(&self) -> &String {
        &self.usdt
    }
}
