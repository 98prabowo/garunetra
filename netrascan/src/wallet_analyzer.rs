use std::{collections::HashSet, fs::create_dir_all};

use chrono::{Datelike, Utc};
use common::TxRecord;

use netrascan::{
    classify::classify_wallet,
    constants::DEFAULT_OFFSET,
    error::{Error, Result},
    heuristics::calculate_score,
    ingest::{EtherscanClient, TransactionClient},
    input::{read_tx_records, read_wallets_from_file},
    model::{WalletFeature, WalletReport},
    output::{append_address_jsonl, append_jsonl, write_json},
};

pub async fn analyze(wallet: &str, api_key: &str, start_block: u64) -> Result<()> {
    let client = EtherscanClient::new(api_key);
    let report = wallet_to_report(&client, wallet, start_block).await?;
    let now = Utc::now();
    let dir = format!(
        "netrascan/data/reports/{:04}-{:02}",
        now.year(),
        now.month()
    );
    let path = format!("{dir}/wallet-report.jsonl");

    create_dir_all(&dir)
        .map_err(|err| Error::from_io(err, "❌ Failed to create wallet report directory"))?;

    append_jsonl(&path, &report)
        .map_err(|e| Error::from_io(e, "❌ Failed to write features to file"))?;

    println!("✅ Address saved to {path}");

    Ok(())
}

pub async fn batch(source: &str, api_key: &str, start_block: u64, output: &str) -> Result<()> {
    let client = EtherscanClient::new(api_key);

    let wallets = read_wallets_from_file(source)
        .map_err(|err| Error::from_io(err, "❌ Failed to read wallet addresses"))?;

    let now = Utc::now();
    let dir = format!("{output}/{:04}-{:02}", now.year(), now.month());
    let path = format!("{dir}/wallet-report.jsonl");

    create_dir_all(&dir)
        .map_err(|err| Error::from_io(err, "❌ Failed to create wallet report directory"))?;

    let mut success_count = 0;
    let mut fail_count = 0;

    for wallet in wallets {
        match wallet_to_report(&client, &wallet, start_block).await {
            Ok(report) => match append_jsonl(&path, &report) {
                Ok(_) => {
                    println!("✅ Report saved to {path} for {wallet}");
                    success_count += 1;
                }
                Err(err) => {
                    eprintln!("❌ Failed to write report for {wallet}: {err}");
                    fail_count += 1;
                }
            },
            Err(_) => {
                eprintln!("⚠️  Skipped wallet {wallet} due to fetch/parse error.");
                fail_count += 1;
            }
        }
    }

    println!("📦 Batch complete. Success: {success_count}, Failed: {fail_count}");

    Ok(())
}

pub async fn crawl(source: &str, api_key: &str, start_block: u64, output: &str) -> Result<()> {
    let client = EtherscanClient::new(api_key);

    let txs = client
        .fetch_all_token_transfer(source, start_block, DEFAULT_OFFSET)
        .await?;

    if txs.is_empty() {
        return Err(Error::EmptyTransaction(source.to_string()));
    }

    let now = Utc::now();
    let dir = format!("{output}/{:04}-{:02}", now.year(), now.month());
    let path = format!("{dir}/wallet-address.jsonl");

    let mut users: HashSet<String> = txs.into_iter().map(|tx| tx.from).collect();

    let existing_wallet: HashSet<String> = read_wallets_from_file(&path)
        .unwrap_or_default()
        .into_iter()
        .collect();

    users.retain(|addr| !existing_wallet.contains(addr));

    if users.is_empty() {
        println!("ℹ️ No new wallet addresses to save (all previously crawled).");
        return Ok(());
    }

    println!("💳 Discovered {} new wallet addresses", users.len());

    create_dir_all(&dir)
        .map_err(|err| Error::from_io(err, "❌ Failed to create wallet addresses directory"))?;

    append_address_jsonl(&path, &users)
        .map_err(|err| Error::from_io(err, "❌ Failed to write wallet addresses"))?;

    println!("✅ Address saved to {path}");

    Ok(())
}

pub async fn train(source: &str, api_key: &str, start_block: u64, output: &str) -> Result<()> {
    let client = EtherscanClient::new(api_key);

    let wallets = read_wallets_from_file(source)
        .map_err(|err| Error::from_io(err, "❌ Failed to read wallet addresses"))?;

    if wallets.is_empty() {
        println!("ℹ️ No wallet addresses found in {source}");
        return Ok(());
    }

    let now = Utc::now();
    let dir = format!("{output}/{:04}-{:02}", now.year(), now.month());
    create_dir_all(&dir)
        .map_err(|err| Error::from_io(err, "❌ Failed to create training model directory"))?;

    let path = format!("{dir}/training-data.jsonl");
    let mut success_count = 0;
    let mut fail_count = 0;

    for (index, wallet) in wallets.iter().enumerate() {
        println!("🔍 [{index:03}] Processing wallet: {wallet}");

        match wallet_to_feature(&client, wallet, start_block).await {
            Ok(feature) => match append_jsonl(&path, &feature) {
                Ok(_) => {
                    println!("✅ Training data saved to {path} for {wallet}");
                    success_count += 1;
                }
                Err(err) => {
                    eprintln!("❌ Failed to write training data for {wallet}: {err}");
                    fail_count += 1;
                }
            },
            Err(_) => {
                eprintln!("⚠️  Skipped wallet {wallet} due to fetch/parse error.");
                fail_count += 1;
            }
        }
    }

    println!("📦 Generated training-ready data completed. Success: {success_count}, Failed: {fail_count}");

    Ok(())
}

pub async fn fetch(source: &str, api_key: &str, start_block: u64, output: &str) -> Result<()> {
    let client = EtherscanClient::new(api_key);

    let txs = client
        .fetch_all_token_transfer(source, start_block, DEFAULT_OFFSET)
        .await?;

    if txs.is_empty() {
        return Err(Error::EmptyTransaction(source.to_string()));
    }

    let records: Vec<TxRecord> = txs.into_iter().map(|tx| tx.into()).collect();

    match write_json(output, &records) {
        Ok(_) => println!("✅ Transactions saved to {output}"),
        Err(err) => eprintln!("❌ Failed to write transactions: {err}"),
    }

    Ok(())
}

pub async fn score(input: &str) -> Result<()> {
    let records =
        read_tx_records(input).map_err(|err| Error::from_io(err, "❌ Failed to load records"))?;

    let score = calculate_score(&records);
    let category = classify_wallet(score);

    println!("📄 Wallet classification result:");
    println!("  Score: {score:.2}");
    println!("  Category: {category:?}");

    Ok(())
}

async fn wallet_to_report<C: TransactionClient>(
    client: &C,
    wallet: &str,
    start_block: u64,
) -> Result<WalletReport> {
    let txs = client
        .fetch_all_token_transfer(wallet, start_block, DEFAULT_OFFSET)
        .await?;

    if txs.is_empty() {
        return Err(Error::EmptyTransaction(wallet.to_string()));
    }

    let records: Vec<TxRecord> = txs.into_iter().map(|tx| tx.into()).collect();

    let score = calculate_score(&records);
    let category = classify_wallet(score);

    Ok(WalletReport::new(wallet, score, category, &records))
}

async fn wallet_to_feature<C: TransactionClient>(
    client: &C,
    wallet: &str,
    start_block: u64,
) -> Result<WalletFeature> {
    let txs = client
        .fetch_all_token_transfer(wallet, start_block, DEFAULT_OFFSET)
        .await?;

    if txs.is_empty() {
        return Err(Error::EmptyTransaction(wallet.to_string()));
    }

    let records: Vec<TxRecord> = txs.into_iter().map(|tx| tx.into()).collect();

    Ok(WalletFeature::from_records(wallet, &records))
}
