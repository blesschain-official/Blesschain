// blesschain-runtime/src/lib.rs

#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;

// FRAME & pallets
use frame_support::{
    construct_runtime,
    parameter_types,
    traits::{ConstU32, ConstU64, Everything},
};
use frame_system as system;

// Core primitives
use sp_core::{H256, OpaqueMetadata};
use sp_runtime::{
    generic,
    traits::{BlakeTwo256, IdentifyAccount, Verify, Block as BlockT},
    MultiSignature, MultiAddress,
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult,
    generic::LazyBlock,
};
use sp_version::RuntimeVersion;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_api::{impl_runtime_apis, Core as CoreApi};
use sp_inherents::{CheckInherentsResult, InherentData};
use sp_consensus_aura::SlotDuration;
use sp_block_builder::BlockBuilder as BlockBuilderApi;
use sp_consensus_aura::AuraApi as AuraRuntimeApi;

// ────────────────────────────────────────────────────────────
// Basic Types
// ────────────────────────────────────────────────────────────

pub type Signature = MultiSignature;
pub type AccountPublic = <Signature as Verify>::Signer;

pub type AccountId = <AccountPublic as IdentifyAccount>::AccountId;
pub type BlockNumber = u32;
pub type Balance = u128;
pub type Index = u32;
pub type Hash = H256;

pub type Address = MultiAddress<AccountId, ()>;

pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

pub type SignedExtra = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
);
pub type UncheckedExtrinsic =
    generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

pub type Block = generic::Block<Header, UncheckedExtrinsic>;

// ────────────────────────────────────────────────────────────
// Runtime Version
// ────────────────────────────────────────────────────────────

pub const VERSION: sp_version::RuntimeVersion = sp_version::RuntimeVersion {
    spec_name: sp_runtime::create_runtime_str!("blesschain"),
    impl_name: sp_runtime::create_runtime_str!("blesschain"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 1,
    transaction_version: 1,
    system_version: 1,
    apis: sp_version::ApisVec::Borrowed(&[]),   // ← FIXED
};

// ────────────────────────────────────────────────────────────
// Constants
// ────────────────────────────────────────────────────────────

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    pub const SS58Prefix: u16 = 42;
    pub const ExistentialDeposit: Balance = 1;

    // 7 seconds per block：Slot = 7000 ms, MinimumPeriod = 3500
    pub const MinimumPeriod: u64 = 3_500;
}

// ────────────────────────────────────────────────────────────
// FRAME System
// ────────────────────────────────────────────────────────────

impl system::Config for Runtime {
    type BaseCallFilter = Everything;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;

    type AccountId = AccountId;
    type Lookup = sp_runtime::traits::IdentityLookup<AccountId>;
    type Block = Block;
    type Hash = Hash;
    type Hashing = BlakeTwo256;
    type Nonce = Index;
    type BlockHashCount = BlockHashCount;

    type DbWeight = ();
    type BlockWeights = ();
    type BlockLength = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;

    type OnNewAccount = ();
    type OnKilledAccount = ();

    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;

    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;

    type RuntimeTask = ();
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
    type ExtensionsWeightInfo = ();
}

// ────────────────────────────────────────────────────────────
// Balances
// ────────────────────────────────────────────────────────────

impl pallet_balances::Config for Runtime {
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;

    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;

    type MaxLocks = ConstU32<50>;
    type MaxReserves = ConstU32<0>;
    type ReserveIdentifier = [u8; 8];

    type WeightInfo = ();
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type DoneSlashHandler = ();
}

// ────────────────────────────────────────────────────────────
// Timestamp
// ────────────────────────────────────────────────────────────

impl pallet_timestamp::Config for Runtime {
    type Moment = u64;
    type MinimumPeriod = MinimumPeriod;
    type OnTimestampSet = ();
    type WeightInfo = ();
}

// ────────────────────────────────────────────────────────────
// Aura
// ────────────────────────────────────────────────────────────

impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
    type MaxAuthorities = ConstU32<32>;
    type DisabledValidators = ();
    type AllowMultipleBlocksPerSlot = ();
    // 7 seconds per block
    type SlotDuration = ConstU64<7000>;
}

// ────────────────────────────────────────────────────────────
// Construct Runtime
// ────────────────────────────────────────────────────────────

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: system,
        Timestamp: pallet_timestamp,
        Balances: pallet_balances,
        Aura: pallet_aura,
    }
);

// ────────────────────────────────────────────────────────────
// Executive
// ────────────────────────────────────────────────────────────

pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;

