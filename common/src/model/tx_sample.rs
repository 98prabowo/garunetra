use chrono::NaiveDateTime;

pub struct TxSample {
    pub from: String,
    pub to: String,
    pub token_symbol: String,
    pub timestamp: NaiveDateTime,
    pub value: f64,
    pub usd_value: Option<f64>,
    pub gas_fee_eth: Option<f64>,
    pub contract_address: String,
    pub hash: String,
}
