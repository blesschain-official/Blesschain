//! Main runtime definition for BlessChain.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]

use sp_api::{impl_runtime_apis, BlockT};
use sp_version::{NativeVersion, RuntimeVersion};
use sp_runtime::{
    generic,
    traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Verify, Checkable},
    MultiSignature, MultiAddress, create_runtime_str,
    transaction_validity::TransactionValidityError,
};
use frame_support::{
    construct_runtime,
    traits::{ConstU128, ConstU64, ConstU32, ConstU16, ConstU8, Everything},
    weights::IdentityFee,
};
use frame_system::limits::{BlockWeights, BlockLength};

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

pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Address = MultiAddress<AccountId, ()>;
pub type Header = generic::Header<u32, BlakeTwo256>;
pub type BlockNumber = u32;
pub type Index = u32;

pub type SignedExtra = (
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

pub type BlessUncheckedExtrinsic = generic::UncheckedExtrinsic<
    Address,
    RuntimeCall,
    Signature,
    SignedExtra,
>;

pub type Block = generic::Block<Header, BlessUncheckedExtrinsic>;
pub type UncheckedExtrinsic = BlessUncheckedExtrinsic;

#[cfg(feature = "std")]
pub const WASM_BINARY: Option<&[u8]> = None;
#[cfg(not(feature = "std"))]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

impl frame_system::Config for Runtime {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Index = Index;
    type BlockNumber = BlockNumber;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<AccountId>;
    type Header = Header;
    type BlockHashCount = ConstU32<250>;
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
    type OperationalFeeMultiplier = ConstU8<5>;
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
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        TransactionPayment: pallet_transaction_payment::{Pallet, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Aura: pallet_aura::{Pallet, Config<T>},
    }
);

type ExecutiveType = frame_executive::Executive<
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
            ExecutiveType::execute_block(block);
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            ExecutiveType::initialize_block(header);
        }
    }
}

// ✅ Final fix for Checkable trait bound
impl sp_runtime::traits::Checkable<frame_system::ChainContext<Runtime>> for BlessUncheckedExtrinsic {
    type Checked = Self;
    fn check(
        self,
        _context: &frame_system::ChainContext<Runtime>,
    ) -> Result<Self::Checked, TransactionValidityError> {
        Ok(self)
    }
}
