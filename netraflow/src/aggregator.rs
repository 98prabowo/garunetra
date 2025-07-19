use std::collections::HashMap;

use common::model::FlowSummary;
use netrascan::classification::TxClassified;

pub fn summarize_block(classified: &[TxClassified]) -> FlowSummary {
    let totals = classified.iter().fold(HashMap::new(), |mut acc, tx| {
        *acc.entry(tx.category).or_default() += tx.value;
        acc
    });

    let (block_number, timestamp) = classified
        .first()
        .map(|tx| (tx.block_number, tx.timestamp))
        .unwrap_or((0, 0));

    FlowSummary {
        block_number,
        timestamp,
        category_totals: totals,
        tx_count: classified.len(),
    }
}
