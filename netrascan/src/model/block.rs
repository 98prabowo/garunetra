use serde::Deserialize;

use crate::{classify::classify_tx, heuristics::Heuristics};

use super::{Tx, TxClassified};

#[derive(Debug, Deserialize)]
pub struct Block {
    pub number: String,
    pub timestamp: String,
    pub transactions: Vec<Tx>,
}

impl Block {
    pub fn classify(self, heuristics: &Heuristics) -> Vec<TxClassified> {
        let block_number =
            u64::from_str_radix(self.number.trim_start_matches("0x"), 16).unwrap_or(0);

        self.transactions
            .into_iter()
            .filter_map(|tx| {
                let category = classify_tx(&tx, heuristics);
                tx.classify(category, block_number, &self.timestamp)
            })
            .collect()
    }
}
