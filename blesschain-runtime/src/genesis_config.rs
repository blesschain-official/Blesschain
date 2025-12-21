//! BlessChain minimal genesis configuration.

use crate::{Runtime, types::*};
use pallet_balances::GenesisConfig as BalancesGenesisConfig;
use frame_system::GenesisConfig as SystemGenesisConfig;

/// Genesis configuration for the minimal BlessChain runtime.
/// Only `System` + `Balances` are included.
#[derive(Clone, Eq, PartialEq)]
pub struct BlesschainGenesisConfig;

impl BlesschainGenesisConfig {
    /// Generate a minimal genesis config.
    /// 
    /// - `initial_accounts`: list of (AccountId, Balance)
    /// - `root_key`: sudo / governance key (if needed later)
    pub fn genesis(
        initial_accounts: Vec<(AccountId, Balance)>,
    ) -> (
        SystemGenesisConfig<Runtime>,
        BalancesGenesisConfig<Runtime>,
    ) {
        // System config — minimal, no fields except `_config`
        let system_genesis = SystemGenesisConfig::<Runtime> {
            _config: Default::default(),
        };

        // Balances pallet: map account → balance
        let balances_genesis = BalancesGenesisConfig::<Runtime> {
            balances: initial_accounts,
            ..Default::default()
        };

        (system_genesis, balances_genesis)
    }
}

