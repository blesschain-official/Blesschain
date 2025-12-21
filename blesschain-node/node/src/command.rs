//! CLI/command wiring for BlessChain node.
use std::path::PathBuf;
use sc_cli::{Subcommand, RuntimeVersionCmd, ChainSpec, CliConfiguration};
use clap::Parser;

use sc_service::config::DatabaseSource;
use sc_service::{config::Configuration, ChainType};
use structopt::StructOpt;

// Re-export runtime types for the CLI.
use blesschain_runtime::RuntimeVersion as RuntimeVer;
use blesschain_runtime::RuntimeGenesisConfig;

/// Your CLI definition. Adapt fields as you like (ports, base paths, etc).
#[derive(Debug, Parser)]
#[clap(name = "blesschain-node", about = "BlessChain node")]
pub struct Cli {
    /// Path to chain spec JSON file
    #[clap(long)]
    pub chain_spec: Option<PathBuf>,

    /// Subcommand to run (check-block, build-spec, export-blocks, export-state, import-blocks, purge-chain, revert, run, etc)
    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,
}

impl Cli {
    pub fn from_args() -> Self {
        Cli::parse()
    }
}

/// Provide chain spec loader. Adapt names to your chain_spec module.
pub fn load_spec(id: &str, path: Option<PathBuf>) -> Result<Box<dyn sc_service::ChainSpec>, String> {
    match id {
        "" | "dev" => Ok(Box::new(crate::chain_spec::development_config()?)),
        "local" => Ok(Box::new(crate::chain_spec::local_testnet_config()?)),
        _ => {
            if let Some(p) = path {
                Ok(Box::new(sc_service::ChainSpec::from_json_file(p)?))
            } else {
                Err("Unknown chain spec id and no path given".into())
            }
        }
    }
}

/// Top-level command entrypoint used by `main`.
pub fn run() -> sc_service::error::Result<()> {
    let cli = Cli::from_args();

    // If subcommand present, handle special commands (build-spec, check-block, etc)
    match &cli.subcommand {
        Some(Subcommand::BuildSpec(cmd)) => {
            // BuildSpec requires a ChainSpec. Use default dev chain spec if none provided.
            let chain_spec = load_spec("dev", cli.chain_spec.clone()).map_err(|e| sc_service::error::Error::Other(e))?;
            cmd.run(chain_spec, sc_service::config::Configuration::default())?;
            return Ok(());
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            // Usually CheckBlock is async and needs a client + import queue. Implement when needed.
            return Err(sc_service::error::Error::Other("CheckBlock not wired in this template".into()));
        }
        Some(Subcommand::Run(cmd)) | None => {
            // Normal node run path
            // Build configuration from args (this leverages sc_cli & sc_service machinery in upstream)
            // For a simple approach, create a default Configuration then call service::new_full.
            let mut config = Configuration::default();
            // You should override config fields from CLI args here (chain_spec path, role, ports, etc).
            // For now, assume dev:
            config.chain_spec = load_spec("dev", cli.chain_spec.clone()).map_err(|e| sc_service::error::Error::Other(e))?;
            // Start the full node:
            let mut task_manager = crate::service::new_full(&config)?;
            // TaskManager runs until exit; usually you exit when TaskManager.join() returns.
            // Here we just keep running until ctrl-c.
            task_manager.future().wait().unwrap_or_else(|_| ());
            return Ok(());
        }
        Some(other) => {
            return Err(sc_service::error::Error::Other(format!("Subcommand {:?} not implemented in template", other)));
        }
    }
}

