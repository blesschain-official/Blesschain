//! BlessChain minimal node (standalone mock runner)
//! Purpose: give you visible "blocks" so you can verify the node loop works.

mod service;
mod chain_spec;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "blesschain-node")]
#[command(about = "BlessChain minimal node (mock blocks)", long_about = None)]
struct Cli {
    /// Chain to run (placeholder, e.g., dev)
    #[arg(long, default_value = "dev")]
    chain: String,

    /// Seconds between mock blocks
    #[arg(long, default_value_t = 2u64)]
    block_interval: u64,
}

fn main() {
    let cli = Cli::parse();
    println!("🚀 Starting BlessChain minimal node ...");
    chain_spec::development_config();

    // 启动“模拟出块”循环
    service::run(cli.block_interval);
}

