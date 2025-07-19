mod cli;
mod controller;

use clap::Parser;
use cli::{Cli, Command};
use netracrawl::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::ScanLatest { rpc } => controller::scan_latest(&rpc).await?,
        Command::ScanBlock { block_number, rpc } => {
            controller::scan_block(block_number, &rpc).await?
        }
    }

    Ok(())
}
