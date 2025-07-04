use chrono::{TimeZone, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::PriceLookupFn;

use super::{TxResponse, TxSample};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TxRecord {
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

impl TxRecord {
    pub fn to_sample(&self, price_lookup: Option<PriceLookupFn>) -> Option<TxSample> {
        let timestamp = self.time_stamp
            .parse::<i64>()
            .ok()
            .and_then(|ts| Utc.timestamp_opt(ts, 0).single())?
            .naive_utc();

        let value_raw: f64 = self.value.parse().ok()?;
        let value_eth = value_raw / 1e18;

        let gas_fee_eth = match (&self.gas_price, &self.gas_used) {
            (Some(gp), Some(gu)) => {
                let gas_price: f64 = gp.parse().ok()?;
                let gas_used: f64 = gu.parse().ok()?;
                Some((gas_price * gas_used) / 1e18)
            }
            _ => None,
        };

        let usd_value = price_lookup
            .and_then(|lookup| lookup(&self.token_symbol, timestamp.and_utc().timestamp()))
            .map(|price| price * value_eth);

        let sample = TxSample {
            from: self.from.clone(),
            to: self.to.clone(),
            token_symbol: self.token_symbol.clone(),
            timestamp,
            value: value_eth,
            usd_value,
            gas_fee_eth,
            contract_address: self.contract_address.clone(),
            hash: self.hash.clone(),
        };

        Some(sample)
    }
}

impl From<TxResponse> for TxRecord {
    fn from(value: TxResponse) -> Self {
        Self { 
            from: value.from, 
            to: value.to, 
            token_symbol: value.token_symbol, 
            time_stamp: value.time_stamp,
            value: value.value,
            contract_address: value.contract_address,
            gas_price: value.gas_price,
            gas_used: value.gas_used,
            hash: value.hash,
        }
    }
}
