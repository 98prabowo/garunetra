use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use common::model::TxRecord;
use serde_json::{from_reader, from_str};

pub fn read_wallets_from_file<P>(path: P) -> std::io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let wallets = reader
        .lines()
        .filter_map(|line| match line {
            Ok(addr) if !addr.trim().is_empty() => Some(addr.trim().to_string()),
            _ => None,
        })
        .collect();

    Ok(wallets)
}

pub fn read_tx_records(path: &str) -> std::io::Result<Vec<TxRecord>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    if let Ok(records) = from_reader(reader) {
        return Ok(records);
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut records = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let record: TxRecord =
            from_str(&line).map_err(|e| std::io::Error::other(format!("Parse error: {e}")))?;
        records.push(record);
    }

    Ok(records)
}
