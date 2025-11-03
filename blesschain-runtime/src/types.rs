// #[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode, TypeInfo)]
// Runtime types used across BlessChain.

use sp_runtime::MultiSignature;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::crypto::AccountId32;
use sp_std::prelude::*;
use frame_support::dispatch::Dispatchable;
use frame_system::{
    CheckNonZeroSender, CheckSpecVersion, CheckTxVersion, CheckGenesis,
    CheckEra, CheckNonce, CheckWeight,
};
use pallet_transaction_payment::ChargeTransactionPayment;

pub type Signature = MultiSignature;
pub type AccountId = AccountId32;

pub type SignedExtra = (
    CheckNonZeroSender<crate::Runtime>,
    CheckSpecVersion<crate::Runtime>,
    CheckTxVersion<crate::Runtime>,
    CheckGenesis<crate::Runtime>,
    CheckEra<crate::Runtime>,
    CheckNonce<crate::Runtime>,
    CheckWeight<crate::Runtime>,
    ChargeTransactionPayment<crate::Runtime>,
);
