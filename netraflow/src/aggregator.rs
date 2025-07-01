use std::collections::HashMap;

use chrono::{TimeZone, Utc};

use crate::model::ClassifiedWallet;

pub type DailyOutflowSummary = HashMap<String, f64>;

pub fn aggregate_outflow(records: &[ClassifiedWallet]) -> DailyOutflowSummary {
    let mut summary = HashMap::new();

    for wallet in records {
        if wallet.category != "Domestic" {
            continue;
        }

        for tx in &wallet.txs {
            let ts: i64 = tx.time_stamp.parse().unwrap_or(0);
            let datetime = Utc.timestamp_opt(ts, 0).unwrap().date_naive();
            let day = datetime.format("%Y-%m-%d").to_string();

            let amount = match tx.token_symbol.to_uppercase().as_str() {
                "USDT" | "USDC" => 1.0,
                "IDRT" | "BIDR" => 0.00007,
                _ => 0.0,
            };

            *summary.entry(day).or_insert(0.0) += amount;
        }
    }

    summary
}
