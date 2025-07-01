mod classify;
mod cli;
mod heuristic;
mod ingest;
mod input;
mod model;
mod output;

use std::{collections::HashSet, fs::create_dir_all};

use chrono::{Datelike, Utc};
use clap::Parser;
use classify::classify_wallet;
use cli::{Cli, Command};
use common::TxRecord;
use dotenv::dotenv;
use heuristic::calculate_score;
use ingest::{EtherscanClient, TransactionClient};
use input::{read_tx_records, read_wallets_from_file};
use model::{WalletAddress, WalletFeature, WalletReport};
use output::{append_address_jsonl, append_jsonl, write_json};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Command::Analyze { wallet, etherscan_key } => analyze(&wallet, &etherscan_key).await,
        Command::Batch { input, etherscan_key, out } => batch(&input, &etherscan_key, &out).await,
        Command::Crawl { source, etherscan_key, out } => crawl(&source, &etherscan_key, &out).await,
        Command::Train { input, etherscan_key, out } => train(&input, &etherscan_key, &out).await,
        Command::Fetch { wallet, etherscan_key, out } => fetch(&wallet, &etherscan_key, &out).await,
        Command::Score { input } => score(&input).await,
    }
}

async fn analyze(wallet: &str, api_key: &str) {
    let client = EtherscanClient::new(api_key);

    if let Some(report) = wallet_to_report(&client, wallet).await {
        let now = Utc::now();
        let dir = format!("netrascan/data/reports/{:04}-{:02}", now.year(), now.month());
        create_dir_all(&dir).expect("Failed to create wallet report directory");

        let path = format!("{dir}/wallet-report.jsonl");
        match append_jsonl(&path, &report) {
            Ok(_) => println!("✅ Address saved to {path}"),
            Err(err) => eprintln!("❌ Failed to write features to file: {err}"),
        }
    }
}

async fn batch(source: &str, api_key: &str, output: &str) {
    let client = EtherscanClient::new(api_key);

    let wallets = read_wallets_from_file(source).expect("Failed to read wallet addresses");

    let now = Utc::now();
    let dir = format!("{output}/{:04}-{:02}", now.year(), now.month());
    create_dir_all(&dir).expect("Failed to create wallet report directory");

    for wallet in wallets {
        if let Some(report) = wallet_to_report(&client, &wallet).await {
            let path = format!("{dir}/wallet-report.jsonl");
            match append_jsonl(&path, &report) {
                Ok(_) => println!("✅ Report saved to {path}"),
                Err(err) => eprintln!("❌ Failed to write report to file: {err}"),
            }
        }
    }
}

async fn crawl(source: &str, api_key: &str, output: &str) {
    let client = EtherscanClient::new(api_key);

    let txs = match client.fetch_token_transfer(source).await {
        Ok(txs) => txs,
        Err(err) => {
            eprintln!("❌ Error fetching transaction: {err}");
            return;
        }
    };

    let mut users = HashSet::new();
    for tx in txs {
        users.insert(tx.from);
    }

    let now = Utc::now();
    let dir = format!("{output}/{:04}-{:02}", now.year(), now.month());
    create_dir_all(&dir).expect("Failed to create wallet addresses directory");

    let path = format!("{dir}/wallet-address.jsonl");
    match append_address_jsonl(&path, &users) {
        Ok(_) => println!("✅ Address saved to {path}"),
        Err(err) => eprintln!("❌ Failed to write features to file: {err}"),
    }
}

async fn train(source: &str, api_key: &str, output: &str) {
    let client = EtherscanClient::new(api_key);
    let wallets = read_wallets_from_file(source).expect("Failed to read wallet addresses");

    let now = Utc::now();
    let dir = format!("{output}/{:04}-{:02}", now.year(), now.month());
    create_dir_all(&dir).expect("Failed to create training model directory");

    let path = format!("{dir}/training-data.jsonl");
    let mut count = 0;

    for wallet in wallets {
        if let Some(feature) = wallet_to_feature(&client, &wallet).await {
            match append_jsonl(&path, &feature) {
                Ok(_) => count += 1,
                Err(err) => eprintln!("❌ Failed to write training feature to file: {err}"),
            }
        }
    }

    println!("✅ Generated training-ready data for {count} wallets: {path}");
}

async fn fetch(source: &str, api_key: &str, output: &str) {
    let client = EtherscanClient::new(api_key);

    let txs = match client.fetch_token_transfer(source).await {
        Ok(txs) => txs,
        Err(err) => {
            eprintln!("❌ Error fetching transaction: {err}");
            return;
        }
    };
    let records: Vec<TxRecord> = txs
        .into_iter()
        .map(|tx| tx.into())
        .collect();

    match write_json(output, &records) {
        Ok(_) => println!("✅ Transactions saved to {output}"),
        Err(err) => eprintln!("❌ Failed to write transactions: {err}"),
    }
}

async fn score(input: &str) {
    let records = match read_tx_records(input) {
        Ok(r) => r,
        Err(err) => {
            eprintln!("❌ Failed to load records: {err}");
            return;
        }
    };

    let score = calculate_score(&records);
    let category = classify_wallet(score);

    println!("📄 Wallet classification result:");
    println!("  Score: {score:.2}");
    println!("  Category: {category:?}");
}

async fn wallet_to_report(client: &EtherscanClient, wallet: &str) -> Option<WalletReport> {
    let txs = match client.fetch_token_transfer(wallet).await {
        Ok(txs) => txs,
        Err(err) => {
            eprintln!("❌ Error fetching transaction: {err}");
            return None;
        }
    };
    let records: Vec<TxRecord> = txs
        .into_iter()
        .map(|tx| tx.into())
        .collect();
    let score = calculate_score(&records);
    let category = classify_wallet(score);

    Some(WalletReport::new(wallet, score, category, &records))
}

async fn wallet_to_feature(client: &EtherscanClient, wallet: &str) -> Option<WalletFeature> {
    let txs = match client.fetch_token_transfer(wallet).await {
        Ok(txs) => txs,
        Err(err) => {
            eprintln!("❌ Error fetching transaction: {err}");
            return None;
        }
    };
    let records: Vec<TxRecord> = txs
        .into_iter()
        .map(|tx| tx.into())
        .collect();

    Some(WalletFeature::from_records(wallet, &records))
}
