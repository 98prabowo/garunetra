use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "netracrawl")]
#[command(author = "Garunetra Labs")]
#[command(version = "0.1.0")]
#[command(about = "Data client from mainnet and third-party data source")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Scan the latest block and classify its transactions
    ScanLatest {
        #[arg(
            long,
            default_value = "https://rpc.ankr.com/eth/6570a5941c65a2a7deffce485080569aa0bf85c6dc4b0f1dc95002557569af40",
            help = "RPC endpoint to use",
            env = "ETH_RPC_URL"
        )]
        rpc: String,
    },

    /// Scan a specific block by number
    ScanBlock {
        #[arg(help = "Ethereum block number to scan")]
        block_number: u64,

        #[arg(
            long,
            default_value = "https://rpc.ankr.com/eth/6570a5941c65a2a7deffce485080569aa0bf85c6dc4b0f1dc95002557569af40",
            help = "RPC endpoint to use",
            env = "ETH_RPC_URL"
        )]
        rpc: String,
    },
}
