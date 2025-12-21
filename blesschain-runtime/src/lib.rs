#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod types;
pub mod genesis_config;

use alloc::vec::Vec;
use sp_runtime::{generic, traits::Block as BlockT};
use sp_runtime::{
    create_runtime_str,
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiSignature,
};

use sp_version::RuntimeVersion;
use frame_support::traits::Everything;
use sp_api::impl_runtime_apis;

fn metadata_at_version(version: u32) -> Option<sp_core::OpaqueMetadata> {
    Runtime::metadata_at_version(version)
}

// ===----------------------------------------------------------------===
//  Runtime Version
// ===----------------------------------------------------------------===

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("blesschain"),
    impl_name: create_runtime_str!("blesschain"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    system_version: 1,
};

// ===----------------------------------------------------------------===
//  Basic Types
// ===----------------------------------------------------------------===

pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Balance = u128;
pub type Nonce = u32;
pub type BlockNumber = u32;

pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

pub type UncheckedExtrinsic =
    sp_runtime::generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, ()>;

pub type Block = generic::Block<Header, UncheckedExtrinsic>;

pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;

// ===----------------------------------------------------------------===
//  Runtime construction
// ===----------------------------------------------------------------===

#[frame_support::runtime]
mod runtime {
    #[runtime::runtime]
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin
    )]
    pub struct Runtime;

    #[runtime::pallet_index(0)]
    pub type System = frame_system;

    #[runtime::pallet_index(1)]
    pub type Balances = pallet_balances;
}

// ===----------------------------------------------------------------===
//  Configurations
// ===----------------------------------------------------------------===

impl frame_system::Config for Runtime {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type RuntimeOrigin = RuntimeOrigin;
    type Nonce = Nonce;
    type Block = Block;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = sp_runtime::traits::AccountIdLookup<AccountId, ()>;
    type BlockHashCount = frame_support::traits::ConstU32<2400>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = frame_support::traits::ConstU16<42>;
    type OnSetCode = ();
    type RuntimeTask = ();
    type ExtensionsWeightInfo = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
}

impl pallet_balances::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = Balance;
    type DustRemoval = ();
    type ExistentialDeposit = frame_support::traits::ConstU128<1000>;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = frame_support::traits::ConstU32<0>;
    type MaxReserves = frame_support::traits::ConstU32<0>;
    type ReserveIdentifier = [u8; 8];
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
    type FreezeIdentifier = ();
    type MaxFreezes = frame_support::traits::ConstU32<0>;
    type DoneSlashHandler = ();
}

// ===----------------------------------------------------------------===
//  Runtime APIs (MINIMAL)
// ===----------------------------------------------------------------===

impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: <Block as sp_runtime::traits::Block>::LazyBlock) { 
                Executive::execute_block(block);
        }

        fn initialize_block(header: &<Block as sp_runtime::traits::Block>::Header) 
            -> sp_runtime::ExtrinsicInclusionMode 
        {
            Executive::initialize_block(header)
        }
    }
}

