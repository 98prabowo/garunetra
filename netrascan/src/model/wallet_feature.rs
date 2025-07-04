use std::collections::HashSet;

use common::{
    model::{TxRecord, TxSample},
    utils::dummy_price_lookup,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct WalletFeature {
    pub wallet: String,
    pub total_tx: usize,
    pub total_out_usd: f64,
    pub distinct_to_count: usize,
    pub interacts_with_cex: bool,
    pub used_bridges: bool,
    pub mixer_pattern_score: f32,
}

impl WalletFeature {
    pub fn from_records(wallet: impl Into<String>, txs: &[TxRecord]) -> Self {
        let wallet = wallet.into();

        let samples: Vec<TxSample> = txs
            .iter()
            .filter_map(|tx| tx.to_sample(Some(dummy_price_lookup)))
            .collect();

        Self::from_samples(&wallet, &samples)
    }

    fn from_samples(wallet: &str, txs: &[TxSample]) -> Self {
        let mut total_out_usd = 0.0;
        let mut distinct_to = HashSet::new();
        let mut interacts_with_cex = false;
        let mut used_bridges = false;
        let mut small_repeated_tx_count = 0;

        for tx in txs {
            if tx.from != wallet {
                continue;
            }

            println!("💰 USD: {:?}", tx.usd_value);

            if let Some(usd) = tx.usd_value {
                total_out_usd += usd;
                if usd < 10.0 {
                    small_repeated_tx_count += 1;
                }
            }

            distinct_to.insert(tx.to.clone());

            let to_lower = tx.to.to_ascii_lowercase();
            if to_lower.contains("binance")
                || to_lower.contains("kucoin")
                || to_lower.contains("okx")
            {
                interacts_with_cex = true;
            }

            if to_lower.contains("bridge") {
                used_bridges = true;
            }
        }

        let mixer_pattern_score = small_repeated_tx_count as f32 / txs.len().max(1) as f32;

        Self {
            wallet: wallet.to_string(),
            total_tx: txs.len(),
            total_out_usd,
            distinct_to_count: distinct_to.len(),
            interacts_with_cex,
            used_bridges,
            mixer_pattern_score,
        }
    }
}
