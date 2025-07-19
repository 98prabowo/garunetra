use clap::{Parser, Subcommand};

/// 📊 `netraflow`: Real-time Ethereum capital flow tracker
///
/// Monitors on-chain transaction data and aggregates capital movement into
/// categories like Domestic, Foreign, Bridge, and Unknown.
#[derive(Parser)]
#[command(name = "netraflow")]
#[command(author = "Garunetra Labs")]
#[command(version = "0.1.0")]
#[command(about = "Real-time Ethereum capital flow aggregator")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Start real-time monitoring and aggregation of Ethereum transaction flows
    Watch {
        #[arg(
            long,
            default_value = "https://rpc.ankr.com/eth/6570a5941c65a2a7deffce485080569aa0bf85c6dc4b0f1dc95002557569af40",
            help = "Ethereum RPC endpoint (e.g. Ankr, Alchemy, etc.)",
            env = "ETH_RPC_URL"
        )]
        rpc: String,

        #[arg(
            long,
            default_value = "netrascan/data/heuristics/heuristics.json",
            help = "Path to heuristic file (JSON format)"
        )]
        heuristics: String,

        #[arg(
            long,
            default_value = "netraflow/data/flow.jsonl",
            help = "Path to store aggregated flow reports"
        )]
        out: String,
    },

    /// Show latest aggregated capital flow summary
    Summary {
        #[arg(
            long,
            default_value = "netraflow/data/flow.jsonl",
            help = "Path to the flow report file"
        )]
        input: String,

        #[arg(long, default_value_t = 10, help = "Number of latest blocks to show")]
        latest: usize,
    },

    /// Clean old data or initialize output directory
    Init {
        #[arg(
            long,
            default_value = "netraflow/data",
            help = "Data directory to create/clean"
        )]
        dir: String,
    },
}
