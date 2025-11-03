use clap::Parser;
use sc_cli::{RunCmd, KeySubcommand};

/// Minimal BlessChain CLI
#[derive(Debug, Parser)]
#[command(name = "blesschain-node")]
#[command(about = "BlessChain minimal node", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Run the node
    Run(RunCmd),

    /// Access key-related commands
    #[command(flatten)]
    Other(KeySubcommand),
}
