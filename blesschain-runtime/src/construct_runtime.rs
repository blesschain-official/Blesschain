//! Construct the runtime by composing the FRAME pallets.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::{
    constants::*, 
    types::*, 
};
use frame_support::{
    construct_runtime, parameter_types,
    traits::{ConstU32, ConstU64},
};
use frame_system::EnsureRoot;

/// Runtime Version
pub const VERSION: sp_version::RuntimeVersion = sp_version::RuntimeVersion {
    spec_name: sp_runtime::create_runtime_str!("blesschain"),
    impl_name: sp_runtime::create_runtime_str!("blesschain"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};

/// AccountId / Signature / Address is already defined in types.rs
pub type RuntimeOrigin = frame_system::Origin<Runtime>;
pub type RuntimeCall = <Runtime as frame_system::Config>::RuntimeCall;
pub type RuntimeEvent = <Runtime as frame_system::Config>::RuntimeEvent;

parameter_types! {
    pub const BlockHashCount: u64 = 240;
    pub BlockLength: frame_system::limits::BlockLength =
        frame_system::limits::BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(WEIGHT_PER_SECOND);
}

/// FRAME System pallet configuration.
impl frame_system::Config for Runtime {
    /// Base call filter
    type BaseCallFilter = frame_support::traits::Everything;

    /// Block number type
    type Block = Block;

    /// Event type
    type RuntimeEvent = RuntimeEvent;

    /// Origin type
    type RuntimeOrigin = RuntimeOrigin;

    /// Index type
    type Nonce = u32;

    /// Hashing algorithm
    type Hashing = sp_runtime::traits::BlakeTwo256;

    /// Hash type
    type Hash = sp_core::H256;

    /// AccountId
    type AccountId = AccountId;

    /// Lookup mechanism
    type Lookup = sp_runtime::traits::IdentityLookup<AccountId>;

    /// Block hash count
    type BlockHashCount = BlockHashCount;

    /// Block weights
    type BlockWeights = BlockWeights;

    /// Block length
    type BlockLength = BlockLength;

    type Version = VERSION;
    type PalletInfo = PalletInfo;

    /// Maximum consumers
    type MaxConsumers = ConstU32<16>;

    /// DB weight
    type DbWeight = ();

    /// SS58 prefix
    type SS58Prefix = ConstU16<42>;

    type AccountData = pallet_balances::AccountData<Balance>;

    type OnNewAccount = ();
    type OnKilledAccount = ();

    type SystemWeightInfo = ();
    type BlockExecutionWeight = ();
    type ExtrinsicBaseWeight = ();
    type RuntimeCall = RuntimeCall;
    type RuntimeTask = RuntimeTask;
}

/// Balances pallet configuration
parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
    pub const MaxLocks: u32 = 10;
}

impl pallet_balances::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = Balance;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type MaxLocks = MaxLocks;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
    type RuntimeHoldReason = ();
    type FreezeIdentifier = ();
}

/// Construct the runtime — only System + Balances
construct_runtime!(
    pub enum Runtime {
        System: frame_system,
        Balances: pallet_balances,
    }
);

