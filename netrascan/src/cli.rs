use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "netrascan")]
#[command(author = "Garunetra Labs")]
#[command(version = "0.1.0")]
#[command(about = "Behavior-based wallet classifier for Ethereum")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    // =========================
    //      Wallet Interface
    // =========================
    #[command(
        about = "Analyze a single Ethereum wallet",
        long_about = "Scans a wallet using Etherscan data and applies behavioral heuristics to classify it (e.g. Domestic, Foreign, Mixer)."
    )]
    Analyze {
        #[arg(help = "Target wallet address to analyze")]
        wallet: String,

        #[arg(
            long,
            env = "ETHERSCAN_API_KEY",
            help = "Etherscan API key used to fetch transaction data"
        )]
        etherscan_key: String,

        #[arg(long, default_value_t = 0, help = "Start block number for pagination")]
        start_block: u64,
    },

    #[command(
        about = "Run classification on a batch of wallet addresses",
        long_about = "Reads a file containing wallet addresses, analyzes each one using Etherscan data, and outputs classification reports."
    )]
    Batch {
        #[arg(
            value_name = "FILE",
            help = "Path to a file with one wallet address per line"
        )]
        input: String,

        #[arg(
            long,
            env = "ETHERSCAN_API_KEY",
            help = "Etherscan API key used to fetch transaction data"
        )]
        etherscan_key: String,

        #[arg(long, default_value_t = 0, help = "Start block number for pagination")]
        start_block: u64,

        #[arg(
            long,
            default_value = "netrascan/data/reports",
            help = "Directory to store the output JSONL reports"
        )]
        out: String,
    },

    #[command(
        about = "Extract wallet addresses that interact with a known source (e.g. CEX deposit)",
        long_about = "Fetches transactions to the source wallet and extracts unique sender addresses for training or further analysis."
    )]
    Crawl {
        #[arg(long, help = "The known source wallet (e.g. a CEX deposit address)")]
        source: String,

        #[arg(
            long,
            env = "ETHERSCAN_API_KEY",
            help = "Etherscan API key used to fetch transaction data"
        )]
        etherscan_key: String,

        #[arg(long, default_value_t = 0, help = "Start block number for pagination")]
        start_block: u64,

        #[arg(
            long,
            default_value = "netrascan/data/trainings",
            help = "Directory to save the crawled wallet address list"
        )]
        out: String,
    },

    #[command(
        about = "Convert raw transaction data into model training format",
        long_about = "Reads a list of wallet addresses, fetches their transaction histories, and extracts behavioral features into a JSONL dataset for training."
    )]
    Train {
        #[arg(long, help = "Path to file containing wallet addresses (one per line)")]
        input: String,

        #[arg(
            long,
            env = "ETHERSCAN_API_KEY",
            help = "Etherscan API key used to fetch transaction data"
        )]
        etherscan_key: String,

        #[arg(long, default_value_t = 0, help = "Start block number for pagination")]
        start_block: u64,

        #[arg(
            long,
            default_value = "netrascan/data/trainings",
            help = "Directory to write the training dataset JSONL"
        )]
        out: String,
    },

    #[command(
        about = "Download raw transactions for a wallet",
        long_about = "Fetches and saves raw transaction data (token transfers) for the specified wallet."
    )]
    Fetch {
        #[arg(help = "Wallet address to fetch transaction history for")]
        wallet: String,

        #[arg(
            long,
            env = "ETHERSCAN_API_KEY",
            help = "Etherscan API key used to fetch transaction data"
        )]
        etherscan_key: String,

        #[arg(long, default_value_t = 0, help = "Start block number for pagination")]
        start_block: u64,

        #[arg(
            long,
            default_value = "netrascan/data/raw/transactions.json",
            help = "Output file path for the fetched transaction JSON"
        )]
        out: String,
    },

    #[command(
        about = "Score and classify a wallet based on local transaction file",
        long_about = "Reads a JSON or JSONL file of token transfers, scores it using heuristics, and outputs a classification result."
    )]
    Score {
        #[arg(
            value_name = "FILE",
            help = "Path to file containing transaction records for a single wallet"
        )]
        input: String,
    },

    // =========================
    //      Tx Inteface
    // =========================
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

    /// Classify a single transaction manually
    Classify {
        #[arg(help = "Transaction hash to classify")]
        tx_hash: String,

        #[arg(
            long,
            default_value = "https://rpc.ankr.com/eth/6570a5941c65a2a7deffce485080569aa0bf85c6dc4b0f1dc95002557569af40",
            help = "RPC endpoint to use",
            env = "ETH_RPC_URL"
        )]
        rpc: String,
    },

    /// List known bridge and CEX addresses
    ListHeuristics {
        #[arg(long, help = "Show only bridge addresses")]
        bridge_only: bool,

        #[arg(long, help = "Show only CEX addresses")]
        cex_only: bool,
    },
}
