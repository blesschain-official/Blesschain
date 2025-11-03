construct_runtime!(
	pub enum Runtime where
		Block = crate::types::Block,
		NodeBlock = crate::types::Block,
		UncheckedExtrinsic = crate::types::UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>, GenesisBuild},
		Aura: pallet_aura::{Pallet, Config<T>}, 
	}
);



pub use frame_support::dispatch::{DispatchInfo, PostDispatchInfo};

pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;

pub use runtime_types::frame_system;
pub use runtime_types::pallet_balances;

pub use crate::{RuntimeCall, RuntimeOrigin, RuntimeEvent};
