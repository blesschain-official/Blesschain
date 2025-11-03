mod command;
mod service;
mod cli;

fn main() {
    let cli = cli::Cli::parse();
    if let Err(e) = command::run(cli) {
        eprintln!("Error: {:?}", e);
    }
}

