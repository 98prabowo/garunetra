use std::{
    fs::{create_dir_all, File},
    io::{BufRead, BufReader},
    path::Path,
    time::Duration,
};

use common::{
    model::{FlowSummary, TxCategory},
    utils::Heuristics,
};
use netracrawl::ethereum::EthereumClient;
use netrascan::classification::classify_block;
use serde_json::from_str;

use crate::{
    aggregator::summarize_block,
    constants::REQUEST_PERIOD,
    error::{Error, Result},
    monitor::FlowMonitor,
    storage::save_summary,
};

pub async fn run<P>(rpc: &str, heuristics_path: P, out_path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut latest_seen = 0;

    if !heuristics_path.as_ref().exists() {
        return Err(Error::HeuristicsNotFound(
            heuristics_path.as_ref().to_string_lossy().into_owned(),
        ));
    }

    let heuristics = Heuristics::load(heuristics_path)?;
    let mut flow_monitor = FlowMonitor::new(10);

    loop {
        let client = EthereumClient::new(rpc);
        let block_number = client.get_latest_block_number().await?;

        if block_number > latest_seen {
            let block = client.get_block_by_number(block_number).await?;
            let classified = classify_block(block, &heuristics);

            let summary = summarize_block(&classified);
            save_summary(&summary, &out_path)?;

            let (delta, alerts) = flow_monitor.push(summary);

            if let Some(delta) = delta {
                println!("Δ Flow @ block {}: {:?}", delta.block_number, delta.deltas);
            }

            for alert in alerts {
                alert.report();
            }

            if let Some(avg) = flow_monitor.avg_flow(&TxCategory::Foreign) {
                println!("📊 Avg Foreign flow: {:.2} ETH", avg as f64 / 1e18);
            }

            if let Some(block) = flow_monitor.latest_block() {
                println!("🔁 Monitoring up to block {block}");
            }

            flow_monitor.print_summary();

            println!("\n======================================================================\n");

            latest_seen = block_number;
        }

        tokio::time::sleep(Duration::from_secs(REQUEST_PERIOD)).await;
    }
}

pub fn print_latest_summary<P>(path: P, latest: usize) -> Result<()>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let lines: Vec<_> = reader.lines().map_while(|line| line.ok()).collect();

    let count = lines.len().min(latest);

    for line in lines.iter().rev().take(count).rev() {
        let summary: FlowSummary = from_str(line)?;
        println!("{summary:#?}");
    }

    Ok(())
}

pub fn init_directory<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    create_dir_all(path)?;
    Ok(())
}
