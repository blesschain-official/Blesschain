//! BlessChain CLI definition (minimal)

use sc_cli::{SubstrateCli, RunCmd};
use sc_service::ChainSpec;

#[derive(Debug, clap::Parser)]
#[command(name = "blesschain-node")]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<RunCmd>,
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "BlessChain Node".into()
    }

    fn impl_version() -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn description() -> String {
        "BlessChain minimal node".into()
    }

    fn author() -> String {
        "BlessChain Team".into()
    }

    fn support_url() -> String {
        "https://blesschain.com".into()
    }

    fn copyright_start_year() -> i32 {
        2025
    }

    fn load_spec(&self, _: &str) -> Result<Box<dyn ChainSpec>, String> {
        Ok(Box::new(crate::chain_spec::development_config()?))
    }
}

