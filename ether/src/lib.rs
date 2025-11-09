//! # ether
//!
//! A Rust library for interacting with the Ethereum blockchain.

mod result;
mod usdt;
pub(crate) mod weth9;

pub(crate) const PUBLIC_RPC_URL: &str = "https://ethereum-rpc.publicnode.com";

pub use result::{EtherError, Result};
pub use usdt::Usdt;
