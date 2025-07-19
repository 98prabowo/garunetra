use serde::Deserialize;

use super::TxEthereum;

#[derive(Debug, Deserialize)]
pub struct BlockEthereum {
    pub number: String,
    pub timestamp: String,
    pub transactions: Vec<TxEthereum>,
}
