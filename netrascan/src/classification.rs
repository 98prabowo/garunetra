use common::{
    model::{BlockEthereum, TxCategory, TxEthereum, WalletType},
    utils::Heuristics,
};

pub struct TxClassified {
    pub hash: String,
    pub from: String,
    pub to: Option<String>,
    pub value: u128,
    pub category: TxCategory,
    pub block_number: u64,
    pub timestamp: u64,
}

impl TxClassified {
    pub fn from_tx(
        tx: TxEthereum,
        category: TxCategory,
        block_number: u64,
        timestamp: impl Into<String>,
    ) -> Option<Self> {
        let value = u128::from_str_radix(tx.value.trim_start_matches("0x"), 16).ok()?;
        let timestamp = u64::from_str_radix(timestamp.into().trim_start_matches("0x"), 16).ok()?;

        Some(TxClassified {
            hash: tx.hash,
            from: tx.from,
            to: tx.to,
            value,
            category,
            block_number,
            timestamp,
        })
    }
}

pub fn classify_block(block: BlockEthereum, heuristics: &Heuristics) -> Vec<TxClassified> {
    let block_number = u64::from_str_radix(block.number.trim_start_matches("0x"), 16).unwrap_or(0);

    block
        .transactions
        .into_iter()
        .filter_map(|tx| {
            let category = tx.categorize(heuristics);
            TxClassified::from_tx(tx, category, block_number, &block.timestamp)
        })
        .collect()
}

pub fn classify_wallet(score: f32) -> WalletType {
    match score {
        s if s >= 0.6 => WalletType::Domestic,
        s if s > 0.2 && s < 0.6 => WalletType::Bridge,
        s if s > -0.5 && s <= 0.2 => WalletType::Mixer,
        s if s <= -0.5 => WalletType::Foreign,
        _ => WalletType::Unknown,
    }
}
