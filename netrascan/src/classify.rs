use common::model::{TxCategory, WalletType};

use crate::{
    heuristics::Heuristics,
    model::Tx,
};

pub fn classify_wallet(score: f32) -> WalletType {
    match score {
        s if s >= 0.6 => WalletType::Domestic,
        s if s > 0.2 && s < 0.6 => WalletType::Bridge,
        s if s > -0.5 && s <= 0.2 => WalletType::Mixer,
        s if s <= -0.5 => WalletType::Foreign,
        _ => WalletType::Unknown,
    }
}

pub fn classify_tx(tx: &Tx, heuristics: &Heuristics) -> TxCategory {
    tx.to
        .as_ref()
        .map(|addr| addr.to_ascii_lowercase())
        .map(|to| {
            if heuristics.is_known_cex(&to) {
                TxCategory::Foreign
            } else if heuristics.is_known_bridge(&to) {
                TxCategory::Bridge
            } else if heuristics.is_known_domestic(&tx.from, &to) {
                TxCategory::Domestic
            } else {
                TxCategory::Foreign
            }
        })
        .unwrap_or(TxCategory::Unknown)
}
