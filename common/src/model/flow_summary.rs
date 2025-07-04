use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::TxCategory;

pub type Wei = u128;

#[derive(Debug, Serialize, Deserialize)]
pub struct FlowSummary {
    pub block_number: u64,
    pub timestamp: u64,
    pub category_totals: HashMap<TxCategory, Wei>,
    pub tx_count: usize,
}
