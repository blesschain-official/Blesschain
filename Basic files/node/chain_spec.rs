use blesschain_runtime::{
    SystemConfig, BalancesConfig as BalancesGenesisConfig, AuraConfig, GenesisConfig,
    AccountId, WASM_BINARY,
};
use sp_core::sr25519;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;

fn get_account_id_from_seed(name: &str) -> AccountId {
    sr25519::Pair::from_string(&format!("//{}", name), None)
        .expect("Seed is valid")
        .public()
        .into()
}

fn get_authority_keys_from_seed(name: &str) -> AuraId {
    sr25519::Pair::from_string(&format!("//{}", name), None)
        .expect("Seed is valid")
        .public()
        .into()
}

pub fn blesschain_genesis() -> GenesisConfig {
    let root_key: AccountId = get_account_id_from_seed("Alice");

    GenesisConfig {
        system: SystemConfig {
            code: WASM_BINARY.unwrap().to_vec(),
            ..Default::default()
        },
        balances: BalancesGenesisConfig {
            balances: vec![
                (root_key.clone(), 1_000_000 * 1_000_000_000_000),
                (get_account_id_from_seed("Alice"), 1 << 60),
                (get_account_id_from_seed("Bob"), 1 << 60),
            ],
        },
        aura: AuraConfig {
            authorities: vec![get_authority_keys_from_seed("Alice")],
        },
    }
}

