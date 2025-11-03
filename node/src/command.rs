//! CLI command dispatcher for BlessChain.

use crate::cli::Cli;

pub fn run(cli: Cli) -> sc_cli::Result<()> {
    match &cli.subcommand {
        None => cli.run(),
        Some(cmd) => cmd.run(),
    }
}

