//! BlessChain minimal runtime that compiles with current Substrate SDK
#![cfg_attr(not(feature = "std"), no_std)]

use sp_core::H256;
use sp_runtime::{
    generic,
    traits::{BlakeTwo256, IdentityLookup},
    AccountId32,
};
use frame_support::{
    construct_runtime, parameter_types,
    traits::{ConstU16, ConstU32, ConstU64, Everything},
};
use frame_system as system;

// --- Declare constants module before use ---
pub mod constants;
use crate::constants::SLOT_DURATION;

// ---------------- Types ----------------
pub type BlockNumber = u32;
pub type Index = u32;
pub type Balance = u128;
pub type AccountId = AccountId32;

// ---------------- System ----------------
impl system::Config for Runtime {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Index = Index;
    type BlockNumber = BlockNumber;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type BlockHashCount = ConstU32<2400>;
    type MaxConsumers = ConstU32<16>;
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
}

// ---------------- Balances ----------------
parameter_types! {
    pub const ExistentialDeposit: u128 = 1;
}
impl pallet_balances::Config for Runtime {
    type Balance = Balance;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ConstU32<10>;
    type MaxReserves = ConstU32<50>;
    type ReserveIdentifier = [u8; 8];
    type HoldIdentifier = ();
    type FreezeIdentifier = ();
    type MaxHolds = ConstU32<0>;
    type MaxFreezes = ConstU32<0>;
}

// ---------------- Timestamp ----------------
impl pallet_timestamp::Config for Runtime {
    type Moment = u64;
    type OnTimestampSet = ();
    /// Minimum period between blocks, set to half of the slot duration (3.5 seconds)
    type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
    type WeightInfo = ();
}

// ---------------- Aura ----------------
use sp_consensus_aura::sr25519;
parameter_types! {
    pub const MaxAuthorities: u32 = 32;
}
impl pallet_aura::Config for Runtime {
    type AuthorityId = sr25519::AuthorityId;
    type DisabledValidators = ();
    type MaxAuthorities = MaxAuthorities;
}

// ---------------- Construct Runtime ----------------
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        Balances: pallet_balances,
        Timestamp: pallet_timestamp,
        Aura: pallet_aura,
    }
);

// ---------------- Mock types for minimal chain ----------------
pub type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
pub type Block = frame_system::mocking::MockBlock<Runtime>;

