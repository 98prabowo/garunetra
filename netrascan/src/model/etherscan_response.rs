use common::model::TxResponse;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EtherscanResponse {
    pub result: Vec<TxResponse>,
}
