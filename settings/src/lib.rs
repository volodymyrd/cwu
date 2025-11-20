use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(default)]
#[serde(rename_all = "snake_case")]
pub struct BlockchainUrls {
    pub rpc_node: String,
    pub block_explorer: Option<String>,
    pub tx_api: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CwuConfig {
    pub network_mode: String,
    pub ether: BlockchainUrls,
    pub tron: BlockchainUrls,
}

pub type Result<T> = std::result::Result<T, ConfigError>;

impl CwuConfig {
    /// Loads the configuration, merging values from multiple layers.
    ///
    /// The loading priority is:
    /// 1. config/default.toml (Lowest)
    /// 2. config/{NETWORK_MODE}.toml (e.g., config/mainnet.toml)
    /// 3. Environment Variables (e.g., CWU_ETHEREUM__RPC_NODE) (Highest)
    pub fn new() -> Result<Self> {
        // Determine the network mode (e.g., "testnet" or "mainnet").
        // Defaults to "testnet" if the environment variable is not set.
        let network_mode = env::var("NETWORK_MODE").unwrap_or_else(|_| "testnet".into());
        Self::load_config(&network_mode, "config/default")
    }

    fn load_config(network_mode: &str, default_path: &str) -> Result<Self> {
        let s = Config::builder()
            .set_override("network_mode", network_mode)?
            // Layer 1: Base Defaults
            .add_source(File::with_name(default_path).required(true))
            // Layer 2: Environment-Specific Overrides
            // This switches all URLs for the network (e.g., testnet -> mainnet)
            .add_source(File::with_name(&format!("config/{}", network_mode)).required(false))
            // Layer 3: Environment Variable Overrides (Highest Priority)
            // Example: CWU_ETHEREUM__RPC_NODE="http://my-secret-node.com"
            .add_source(
                Environment::with_prefix("CWU")
                    .separator("__")
                    .ignore_empty(true),
            )
            .build()?;

        s.try_deserialize()
    }

    pub fn test_new(path_to_test_config: &str) -> Self {
        Self::load_config("testing", path_to_test_config)
            .expect("Failed to load testing configuration")
    }
}
