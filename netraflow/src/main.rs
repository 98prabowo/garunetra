mod aggregator;
mod cli;
mod constants;
mod controller;
mod error;
mod monitor;
mod storage;

use clap::Parser;

use cli::{Cli, Command};
use controller::{init_directory, print_latest_summary, run};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Watch {
            rpc,
            heuristics,
            out,
        } => {
            if let Err(err) = run(&rpc, heuristics, out).await {
                eprintln!("❌ Watch error: {err}");
            }
        }
        Command::Summary { input, latest } => {
            if let Err(err) = print_latest_summary(input, latest) {
                eprintln!("❌ Summary error: {err}");
            }
        }
        Command::Init { dir } => {
            if let Err(err) = init_directory(&dir) {
                eprintln!("❌ Init error: {err}");
            } else {
                println!("✅ Initialized directory: {dir}");
            }
        }
    }
}
