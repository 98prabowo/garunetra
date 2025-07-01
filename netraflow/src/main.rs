mod aggregator;
mod cli;
mod input;
mod model;
mod reporter;

use clap::Parser;

use aggregator::aggregate_outflow;
use cli::Cli;
use input::load_jsonl;
use reporter::export_csv;

fn main() {
    let cli = Cli::parse();
    match cli.command.as_str() {
        "analyze" => {
            let records = load_jsonl(&cli.input_dir);
            let summary = aggregate_outflow(&records);
            export_csv("outflow_by_day.csv", &summary).expect("Failed to write CSV");
        }
        _ => eprintln!("Unsupported command: {}", cli.command),
    }
}
