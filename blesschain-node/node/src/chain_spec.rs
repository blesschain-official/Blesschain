use sc_chain_spec::ChainSpecExtension;
use sc_chain_spec::GenericChainSpec;
use sc_service::ChainType;
use serde::{Deserialize, Serialize};

use blesschain_runtime::{
    AccountId, Balance, Block, RuntimeGenesisConfig,
    BalancesConfig, SystemConfig,
};

/// Type alias
pub type ChainSpec = GenericChainSpec<RuntimeGenesisConfig>;

/// Simple extension type (required by GenericChainSpec)
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Extensions;
impl ChainSpecExtension for Extensions {}

/// Helper: convert seed → AccountId
pub fn get_account_id_from_seed(seed: &str) -> AccountId {
    use sp_core::crypto::Ss58Codec;
    AccountId::from_ss58check(seed).expect("Invalid SS58 in chain spec seed")
}

/// Helper: convert balance
pub fn units(x: u128) -> Balance {
    x * 1_000_000_000_000
}

/// Development chain spec
pub fn development_config() -> Result<ChainSpec, String> {
    Ok(GenericChainSpec::from_genesis(
        // Name
        "BlessChain Development",
        // ID
        "blesschain_dev",
        ChainType::Development,
        move || testnet_genesis(),
        // Bootnodes
        Vec::new(),
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        Some(chain_properties()),
        // Extensions
        Default::default(),
    ))
}

/// Local testnet chain spec
pub fn local_testnet_config() -> Result<ChainSpec, String> {
    Ok(GenericChainSpec::from_genesis(
        "BlessChain Local Testnet",
        "blesschain_local",
        ChainType::Local,
        move || testnet_genesis(),
        Vec::new(),
        None,
        None,
        Some(chain_properties()),
        Default::default(),
    ))
}

/// Create genesis config
fn testnet_genesis() -> RuntimeGenesisConfig {
    // initial endowed accounts
    let initial_accounts: Vec<(AccountId, Balance)> = vec![
        (
            get_account_id_from_seed("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXDBN5uT2emX5Y"), // Alice
            units(1_000_000),
        ),
        (
            get_account_id_from_seed("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZ1gcHb8VdJG6hZ"), // Bob
            units(1_000_000),
        ),
    ];

    RuntimeGenesisConfig {
        system: SystemConfig {
            // Runtime wasm code is automatically filled by node build.rs
            ..Default::default()
        },
        balances: BalancesConfig {
            balances: initial_accounts,
            ..Default::default()
        },
    }
}

/// Chain properties (SS58 address prefix 42 like Substrate)
fn chain_properties() -> sc_chain_spec::Properties {
    let mut props = sc_chain_spec::Properties::new();
    props.insert("tokenSymbol".into(), "BLS".into());
    props.insert("tokenDecimals".into(), 12.into());
    props.insert("ss58Format".into(), 42.into());
    props
}

