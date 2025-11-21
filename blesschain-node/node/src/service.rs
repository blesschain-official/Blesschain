// Minimal BlessChain node service using Aura + Timestamp + Wasm executor.

use crate::cli::Consensus;

use futures::FutureExt;
use std::sync::Arc;

// Runtime
use blesschain_runtime::{Block, Runtime};

// Substrate core imports
use sc_client_api::backend::Backend;
use sc_consensus::DefaultImportQueue;
use sc_consensus::LongestChain;
use sc_network::config::FullNetworkConfiguration;
use sc_service::{
    error::Error as ServiceError, Configuration, TaskManager, TFullBackend, TFullClient,
};
use sc_telemetry::{Telemetry, TelemetryWorker};

// Transaction pool
use sc_transaction_pool::{
    BasicPool,
    FullChainApi,
    Options as TxOptions,
};
// Aura consensus
use sc_consensus_aura::{self, SlotProportion};
use sp_consensus_aura::sr25519::AuthorityPair as AuraPair;

// Timestamp inherent
use sp_timestamp::InherentDataProvider;
use sp_consensus_slots::InherentDataProvider as SlotsInherent;

// Runtime traits
use sp_runtime::traits::Block as BlockT;

// Host functions for wasm executor
type HostFunctions = sp_io::SubstrateHostFunctions;

// Wasm executor (no native)
type Executor = sc_executor::WasmExecutor<HostFunctions>;

// Concrete types
type FullClient = TFullClient<Block, Runtime, Executor>;
type FullBackend = TFullBackend<Block>;
type FullSelectChain = LongestChain<FullBackend, Block>;

// Basic transaction pool type (no Arc here)
type TxPool = BasicPool<FullChainApi<FullClient, Block>, Block>;

/// Partial components returned by `new_partial`.
pub type Service = sc_service::PartialComponents<
    FullClient,
    FullBackend,
    FullSelectChain,
    DefaultImportQueue<Block>,
    TxPool,
    Option<Telemetry>,
>;

/// Build client, backend, tx pool, import queue, etc.
/// This is used both by the main node and by subcommands.
pub fn new_partial(config: &Configuration) -> Result<Service, ServiceError> {
    // Telemetry setup
    let telemetry = config
        .telemetry_endpoints
        .clone()
        .filter(|x| !x.is_empty())
        .map(|endpoints| -> Result<_, sc_telemetry::Error> {
            let worker = TelemetryWorker::new(16)?;
            let telemetry = worker.handle().new_telemetry(endpoints);
            Ok((worker, telemetry))
        })
        .transpose()?;

    // Wasm executor
    let executor = sc_service::new_wasm_executor(&config.executor);

    // Client / backend / keystore / task manager
    let (client, backend, keystore_container, mut task_manager) =
        sc_service::new_full_parts::<Block, Runtime, _>(
            config,
            telemetry.as_ref().map(|(_, t)| t.handle()),
            executor,
        )?;

    let client: Arc<FullClient> = Arc::new(client);

    // Spawn telemetry worker
    let telemetry = telemetry.map(|(worker, telemetry)| {
        task_manager
            .spawn_handle()
            .spawn("telemetry", None, worker.run());
        telemetry
    });

    // Longest chain selection
    let select_chain = LongestChain::new(backend.clone());

    // --- Transaction pool ---
    //
    // NOTE: BasicPool::new_full expects `Options`, but config.transaction_pool is
    //       `TransactionPoolOptions`, so we convert via `.into()`.
    let transaction_pool: tx_pool.clone(
        TxOptions::from(config.transaction_pool.clone()),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_essential_handle(),
        client.clone(),
    );

    // --- Aura import queue ---
    //
    // Import queue only needs block import, client, and inherent data providers.
    // Inherent data providers: Timestamp only (Aura slots come from runtime).
    let import_queue = {
        let client_clone = client.clone();
        let telemetry_handle = telemetry.as_ref().map(|t| t.handle());

        sc_consensus_aura::import_queue::<AuraPair, _, _, _, _, _>(
            sc_consensus_aura::ImportQueueParams {
                block_import: client_clone.clone(),
                justification_import: None,
                client: client_clone.clone(),
                create_inherent_data_providers: move |_, ()| async move {
                    // Provide Timestamp inherent only.
                    Ok((InherentDataProvider::from_system_time(),))
                },
                spawner: &task_manager.spawn_essential_handle(),
                registry: config.prometheus_registry(),
                check_for_equivocation: Default::default(),
                telemetry: telemetry_handle,
                compatibility_mode: Default::default(),
            },
        )?
    };

    Ok(sc_service::PartialComponents {
        client,
        backend,
        task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        other: telemetry,
    })
}

/// Build the full service: network, RPC, tasks, Aura authoring.
pub fn new_full<Network: sc_network::NetworkBackend<Block, <Block as BlockT>::Hash>>(
    config: Configuration,
    _consensus: Consensus,
) -> Result<TaskManager, ServiceError> {
    // Get partial components first
    let partial = new_partial(&config)?;

    let client = partial.client;
    let backend = partial.backend;
    let mut task_manager = partial.task_manager;
    let import_queue = partial.import_queue;
    let keystore_container = partial.keystore_container;
    let select_chain = partial.select_chain;
    let tx_pool = partial.transaction_pool;
    let mut telemetry = partial.other;

    // Wrap transaction pool in Arc (this is what spawn_tasks expects)
    let transaction_pool_for_aura = Arc::new(tx_pool.clone());
    let proposer = sc_basic_authorship::ProposerFactory::new(
        task_manager.spawn_handle(),
        client.clone(),
        transaction_pool_for_aura.clone(),

    );

    // --- Network configuration ---
    let net_config = FullNetworkConfiguration::<Block, <Block as BlockT>::Hash, Network>::new(
        &config.network,
        config
            .prometheus_config
            .as_ref()
            .map(|cfg| cfg.registry.clone()),
    );

    let metrics =
        Network::register_notification_metrics(config.prometheus_config.as_ref().map(|cfg| {
            &cfg.registry
        }));

    let (network, system_rpc_tx, tx_handler_controller, sync_service) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            net_config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            block_announce_validator_builder: None,
            warp_sync_config: None,
            block_relay: None,
            metrics,
        })?;

    // --- RPC builder ---
    let rpc_extensions_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();

        Box::new(move |_| {
            let deps = crate::rpc::FullDeps {
                client: client.clone(),
                pool: pool.clone(),
            };
            crate::rpc::create_full(deps).map_err(Into::into)
        })
    };

    // --- Spawn background tasks ---
    sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        network,
        client: client.clone(),
        keystore: keystore_container.keystore(),
        task_manager: &mut task_manager,
        transaction_pool: transaction_pool.clone(),
        rpc_builder: rpc_extensions_builder,
        backend,
        system_rpc_tx,
        tx_handler_controller,
        sync_service: sync_service.clone(),
        config,
        telemetry: telemetry.as_mut(),
        tracing_execute_block: None,
    })?;

    // --- Aura authoring (only if we have a keystore) ---
    if config.role.is_authority() {
        let proposer = sc_basic_authorship::ProposerFactory::new(
            task_manager.spawn_handle(),
            client.clone(),
            transaction_pool.clone(),
            None,
            telemetry.as_ref().map(|t| t.handle()),
        );

        let slot_duration = sc_consensus_aura::slot_duration(&*client)?;

        let aura_params = sc_consensus_aura::StartAuraParams {
            slot_duration,
            client: client.clone(),
            select_chain,
            block_import: client.clone(),
            proposer_factory: proposer,
            create_inherent_data_providers: move |_, ()| async move {
                Ok((InherentDataProvider::from_system_time(),))
            },
            force_authoring: true,
            backoff_authoring_blocks: None,
            keystore: keystore_container
                .keystore()
                .expect("keystore is present; qed"),
            sync_oracle: sync_service.clone(),
            justification_sync_link: sync_service,
            block_proposal_slot_portion: SlotProportion::new(2f32 / 3f32),
            max_block_proposal_slot_portion: None,
            telemetry: telemetry.as_ref().map(|t| t.handle()),
            compatibility_mode: Default::default(),
        };

        let aura_future = sc_consensus_aura::start_aura(aura_params)?;
        task_manager
            .spawn_essential_handle()
            .spawn_blocking("aura", None, aura_future);
    }

    Ok(task_manager)
}

