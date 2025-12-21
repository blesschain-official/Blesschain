//! Minimal BlessChain node service.
//! Works with the minimal runtime: System + Balances only.

use std::sync::Arc;
use sc_service::{
    config::Configuration, error::Result, BuildBlockParams, TFullBackend, TFullClient,
};
use sp_consensus::BlockOrigin;
use blesschain_runtime::{opaque::Block, Runtime};
use sp_runtime::traits::Header;

/// Create the minimal full client + backend.
pub fn new_full(
    config: &Configuration,
) -> Result<(Arc<TFullClient<Block, Runtime, sc_executor::NativeElseWasmExecutor<ExecutorDispatch>>>, Arc<TFullBackend<Block>>, TaskManager)>
{
    let executor = sc_executor::NativeElseWasmExecutor::<ExecutorDispatch>::new();

    let (client, backend, _, task_manager) =
        sc_service::new_full_parts::<Block, Runtime, _>(config, executor)?;

    Ok((client, backend, task_manager))
}

/// Build a single block manually (for minimal chain / instant seal style).
pub fn build_block(
    client: Arc<TFullClient<Block, Runtime, sc_executor::NativeElseWasmExecutor<ExecutorDispatch>>>,
    backend: Arc<TFullBackend<Block>>,
) -> Result<()> {
    let parent_hash = backend.blockchain().info().best_hash;
    let parent_header = backend.blockchain().header(parent_hash)?.unwrap();

    let params = BuildBlockParams {
        parent_hash,
        parent_number: *parent_header.number(),
        inherent_digests: Default::default(),
        inherent_data: Default::default(),
        max_inherent_data: None,
    };

    let built = client.build_block(params)?;
    let block = built.block;

    backend.block_import().import_block(sc_consensus::ImportBlock {
        origin: BlockOrigin::File,
        block,
        justification: None,
        intermediate: Vec::new(),
        fork_choice: sc_consensus::ForkChoiceStrategy::LongestChain,
    })?;

    Ok(())
}

/// Executor dispatch type
pub struct ExecutorDispatch;
impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
    type ExtendHostFunctions = ();
    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        blesschain_runtime::api::dispatch(method, data)
    }
    fn native_version() -> sc_executor::NativeVersion {
        blesschain_runtime::native_version()
    }
}

/// Create a minimal light client (not recommended for minimal chains, provided only for API compatibility)
pub fn new_light(_config: &Configuration) -> Result<TaskManager> {
    // Light client not supported in minimal runtime
    Err("Light client not supported for BlessChain minimal runtime".into())
}

