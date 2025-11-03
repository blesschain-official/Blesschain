use blesschain_runtime::{
    SystemConfig, BalancesConfig, GenesisConfig, AccountId, WASM_BINARY, Runtime
};
use sp_core::sr25519;
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;

pub fn get_account_id_from_seed(name: &str) -> AccountId {
    sr25519::Pair::from_string(&format!("//{}", name), None)
        .expect("valid static seed")
        .public()
        .into()
}

pub fn get_authority_keys_from_seed(name: &str) -> AuraId {
    sr25519::Pair::from_string(&format!("//{}", name), None)
        .expect("valid static seed")
        .public()
        .into()
}

pub fn blesschain_genesis() -> GenesisConfig {
    let root_key: AccountId = get_account_id_from_seed("Alice");

    GenesisConfig {
        system: SystemConfig {
            code: WASM_BINARY.to_vec(),
            ..Default::default()
        },
        balances: BalancesConfig {
            balances: vec![(root_key.clone(), 1_000_000 * 1_000_000_000_000)],
        },
        aura: AuraConfig {
            authorities: vec![get_authority_keys_from_seed("Alice")],
        },
        aura: Some(Default::default()),
}),
    }
}

