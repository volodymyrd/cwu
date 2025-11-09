use std::fmt::{Display, Formatter};
use std::slice::Iter;

#[derive(Debug)]
pub enum Network {
    Ethereum,
    Tron,
}

impl Display for Network {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Network::Ethereum => write!(f, "Ethereum"),
            Network::Tron => write!(f, "Tron"),
        }
    }
}

impl Network {
    pub const ALL_VARIANTS: [Network; 2] = [Network::Ethereum, Network::Tron];

    pub fn iter() -> Iter<'static, Network> {
        Network::ALL_VARIANTS.iter()
    }
}
