use common::model::TxCategory;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Tx {
    pub hash: String,
    pub from: String,
    pub to: Option<String>,
    pub input: String,
    pub value: String,
}

pub struct TxClassified {
    pub hash: String,
    pub from: String,
    pub to: Option<String>,
    pub value: u128,
    pub category: TxCategory,
    pub block_number: u64,
    pub timestamp: u64,
}

impl Tx {
    pub fn classify(
        self,
        category: TxCategory,
        block_number: u64,
        timestamp: impl Into<String>,
    ) -> Option<TxClassified> {
        let value = u128::from_str_radix(self.value.trim_start_matches("0x"), 16).ok()?;
        let timestamp = u64::from_str_radix(timestamp.into().trim_start_matches("0x"), 16).ok()?;

        Some(TxClassified {
            hash: self.hash,
            from: self.from,
            to: self.to,
            value,
            category,
            block_number,
            timestamp,
        })
    }
}
