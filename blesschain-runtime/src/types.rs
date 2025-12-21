use sp_runtime::{
    generic,
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiAddress,
};

pub type BlockNumber = u32;
pub type Balance = u128;
pub type Nonce = u32;

pub type Signature = sp_runtime::MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

pub type Address = MultiAddress<AccountId, ()>;

pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

