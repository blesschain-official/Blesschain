use sp_core::OpaquePeerId;
use sp_runtime::{
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiSignature, generic,
};
use sp_std::vec::Vec;

pub type BlockNumber = u32;
pub type Balance = u128;
pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Index = u32;
pub type Hash = sp_core::H256;
pub type Moment = u64;
pub type Digest = generic::Digest;
pub type DigestItem = generic::DigestItem<Hash>;

pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, sp_runtime::OpaqueExtrinsic>;

pub type OpaqueBlock = Block;
pub type OpaqueHeader = Header;
pub type PeerId = OpaquePeerId;
