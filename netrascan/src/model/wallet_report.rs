use common::{TxRecord, WalletType};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct WalletReport {
    pub wallet: String,
    pub score: f32,
    pub category: WalletType,
    pub txs: Vec<TxRecord>,
}

impl WalletReport {
    pub fn new(
        wallet: impl Into<String>,
        score: f32,
        category: WalletType,
        txs: &[TxRecord],
    ) -> Self {
        Self { 
            wallet: wallet.into(), 
            score, 
            category, 
            txs: txs.to_vec(),
        }
    }
}
