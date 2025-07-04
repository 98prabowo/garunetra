use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxResponse {
    pub block_number: String,
    pub from: String,
    pub to: String,
    pub token_symbol: String,
    pub time_stamp: String,
    pub value: String,
    pub contract_address: String,
    pub gas_price: Option<String>,
    pub gas_used: Option<String>,
    pub hash: String,
}
