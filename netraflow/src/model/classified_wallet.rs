use serde::Deserialize;

use common::TxRecord;

#[derive(Debug, Deserialize)]
pub struct ClassifiedWallet {
    pub wallet: String,
    pub score: f32,
    pub category: String,
    pub txs: Vec<TxRecord>,
}
