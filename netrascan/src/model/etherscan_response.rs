use common::TxResponse;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EtherscanResponse {
    pub result: Vec<TxResponse>,
}
