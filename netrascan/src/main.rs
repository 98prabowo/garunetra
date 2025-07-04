mod cli;
mod wallet_analyzer;

use clap::Parser;
use cli::{Cli, Command};

use netrascan::{error::Result, tx_analyzer};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Analyze {
            wallet,
            etherscan_key,
            start_block,
        } => wallet_analyzer::analyze(&wallet, &etherscan_key, start_block).await?,
        Command::Batch {
            input,
            etherscan_key,
            start_block,
            out,
        } => wallet_analyzer::batch(&input, &etherscan_key, start_block, &out).await?,
        Command::Crawl {
            source,
            etherscan_key,
            start_block,
            out,
        } => wallet_analyzer::crawl(&source, &etherscan_key, start_block, &out).await?,
        Command::Train {
            input,
            etherscan_key,
            start_block,
            out,
        } => wallet_analyzer::train(&input, &etherscan_key, start_block, &out).await?,
        Command::Fetch {
            wallet,
            etherscan_key,
            start_block,
            out,
        } => wallet_analyzer::fetch(&wallet, &etherscan_key, start_block, &out).await?,
        Command::Score { input } => wallet_analyzer::score(&input).await?,
        Command::ScanLatest { rpc } => tx_analyzer::scan_latest(&rpc).await?,
        Command::ScanBlock { 
            block_number, 
            rpc, 
        } => tx_analyzer::scan_block(block_number, &rpc).await?,
        Command::Classify { 
            tx_hash, 
            rpc,
        } => tx_analyzer::classify(&tx_hash, &rpc).await?,
        Command::ListHeuristics { 
            bridge_only, 
            cex_only 
        } => tx_analyzer::list_heuristics(bridge_only, cex_only).await?,
    }

    Ok(())
}
