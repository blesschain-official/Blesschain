//! Minimal BlessChain RPC configuration.

use std::sync::Arc;
use jsonrpsee::RpcModule;
use sc_rpc_api::DenyUnsafe;
use sc_service::{Configuration, TFullBackend, TFullClient};

use blesschain_runtime::Block;

pub fn create_full<C, B>(
	client: Arc<C>,
	_backend: Arc<B>,
	_deny_unsafe: DenyUnsafe,
	_config: &Configuration,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	C: sc_client_api::ProvideRuntimeApi<Block> + Send + Sync + 'static,
	C::Api: sp_api::Core<Block> + 'static,
	B: sc_client_api::Backend<Block> + 'static,
{
	let module = RpcModule::new(());
	Ok(module)
}

