use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "deviflow", version = "0.1.0", author = "Garunetra Labs")]
pub struct Cli {
    #[arg(short, long, default_value = "analyze")]
    pub command: String,

    #[arg(short, long, help = "Input folder containing DeviGuard JSONL outputs")]
    pub input_dir: String,
}
