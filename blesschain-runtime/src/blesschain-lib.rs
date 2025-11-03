
//! Main runtime definition for BlessChain.

#![cfg_attr(not(feature = "std"), no_std)]

use sp_api::impl_runtime_apis;
use sp_version::{NativeVersion, RuntimeVersion};
use sp_runtime::{
    generic::{UncheckedExtrinsic as GenericUncheckedExtrinsic, Block as GenericBlock},
    traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Verify},
    MultiSignature, MultiAddress, create_runtime_str,
};

use frame_support::{
    construct_runtime, parameter_types,
    traits::{ConstU128, ConstU64, ConstU32, ConstU16, ConstU8, Everything},
    weights::{IdentityFee, Weight},
};

use frame_system::limits::{BlockWeights, BlockLength};
use frame_system::ChainContext;

pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Address = MultiAddress<AccountId, ()>;
pub type BlockNumber = u32;
pub type Index = u32;
pub type Header = sp_runtime::generic::Header<BlockNumber, BlakeTwo256>;

pub type SignedExtra = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

pub type Extrinsic = GenericUncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
pub type Block = GenericBlock<Header, Extrinsic>;

#[cfg(feature = "std")]
pub const WASM_BINARY: Option<&[u8]> = None;
#[cfg(not(feature = "std"))]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: RuntimeVersion {
            spec_name: create_runtime_str!("blesschain"),
            impl_name: create_runtime_str!("blesschain"),
            authoring_version: 1,
            spec_version: 1,
            impl_version: 1,
            apis: RUNTIME_API_VERSIONS,
            transaction_version: 1,
            state_version: 1,
        },
        can_author_with: Default::default(),
    }
}

parameter_types! {
    pub const BlockHashCount: u32 = 250;
    pub BlockWeights: BlockWeights = BlockWeights::simple_max(2 * 1024 * 1024);
    pub BlockLength: BlockLength = BlockLength::max_with_normal_ratio(5 * 1024 * 1024, frame_support::weights::constants::NORMAL_DISPATCH_RATIO);
}

impl frame_system::Config for Runtime {
    type BaseCallFilter = Everything;
    type BlockWeights = BlockWeights;
    type BlockLength = BlockLength;
    type DbWeight = ();
    type Index = Index;
    type BlockNumber = BlockNumber;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<AccountId>;
    type Header = Header;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type AccountData = pallet_balances::AccountData<u128>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type PalletInfo = PalletInfo;
}

impl pallet_balances::Config for Runtime {
    type Balance = u128;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<1>;
    type AccountStore = frame_system::Pallet<Runtime>;
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
    type HoldIdentifier = ();
    type FreezeIdentifier = ();
    type MaxHolds = ();
    type MaxFreezes = ();
    type RuntimeEvent = RuntimeEvent;
}

impl pallet_transaction_payment::Config for Runtime {
    type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<
        pallet_balances::Pallet<Runtime>,
        (),
    >;
    type OperationalFeeMultiplier = ConstU8<1>;
    type WeightToFee = IdentityFee<u128>;
    type FeeMultiplierUpdate = ();
    type LengthToFee = IdentityFee<u128>;
    type RuntimeEvent = RuntimeEvent;
}

impl pallet_timestamp::Config for Runtime {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ConstU64<5000>;
    type WeightInfo = ();
}

impl pallet_aura::Config for Runtime {
    type AuthorityId = sp_consensus_aura::sr25519::AuthorityId;
    type MaxAuthorities = ConstU32<32>;
    type DisabledValidators = ();
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = Extrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        TransactionPayment: pallet_transaction_payment::{Pallet, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Aura: pallet_aura::{Pallet, Config<T>},
    }
);

type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    ()
>;

impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            native_version().runtime_version
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block);
        }

        fn initialize_block(header: &<Block as sp_runtime::traits::Block>::Header) {
            Executive::initialize_block(header);
        }
    }
}
