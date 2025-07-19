use common::model::{BlockEthereum, TxEthereum};
use reqwest::Client;
use serde_json::{from_value, json, Value};

use crate::error::{Error, Result};

pub struct EthereumClient {
    pub client: Client,
    pub url: String,
}

impl EthereumClient {
    pub fn new(url: impl Into<String>) -> Self {
        EthereumClient {
            client: Client::new(),
            url: url.into(),
        }
    }

    pub async fn get_latest_block_number(&self) -> Result<u64> {
        let response = self
            .client
            .post(&self.url)
            .json(&json!({
                "jsonrpc": "2.0",
                "method": "eth_blockNumber",
                "params": [],
                "id": 1
            }))
            .send()
            .await?
            .json::<Value>()
            .await?;

        if let Some(error) = response.get("error") {
            let code = error.get("code").and_then(Value::as_i64).unwrap_or(0);
            let message = error
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or("Unknown error");
            return Err(Error::Rpc(code, message.to_string()));
        }

        let hex = response
            .get("result")
            .ok_or(Error::EmptyBlockNumber)?
            .as_str()
            .ok_or(Error::EmptyBlockNumber)?;

        let block_number = u64::from_str_radix(hex.trim_start_matches("0x"), 16)?;

        Ok(block_number)
    }

    pub async fn get_block_by_number(&self, number: u64) -> Result<BlockEthereum> {
        let hex_block = format!("0x{number:x}");

        let response = self
            .client
            .post(&self.url)
            .json(&json!({
                "jsonrpc": "2.0",
                "method": "eth_getBlockByNumber",
                "params": [hex_block, true],
                "id": 1
            }))
            .send()
            .await?
            .json::<Value>()
            .await?; // Also auto-mapped

        if let Some(error) = response.get("error") {
            let code = error.get("code").and_then(Value::as_i64).unwrap_or(0);
            let message = error
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or("Unknown error");
            return Err(Error::Rpc(code, message.to_string()));
        }

        let value = response.get("result").ok_or(Error::EmptyBlockResult)?;

        from_value(value.clone())
            .map_err(|e| Error::Serde(e, format!("Failed to parse block: {hex_block}")))
    }

    pub async fn get_transaction_by_hash(&self, hash: &str) -> Result<TxEthereum> {
        let response = self
            .client
            .post(&self.url)
            .json(&json!({
                "jsonrpc": "2.0",
                "method": "eth_getTransactionByHash",
                "params": [hash],
                "id": 1
            }))
            .send()
            .await?
            .json::<Value>()
            .await?;

        if let Some(error) = response.get("error") {
            let code = error.get("code").and_then(Value::as_i64).unwrap_or(0);
            let message = error
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or("Unknown error");
            return Err(Error::Rpc(code, message.to_string()));
        }

        let tx = response.get("result").ok_or(Error::EmptyBlockResult)?;

        if tx.is_null() {
            return Err(Error::TxNotFound(hash.to_string()));
        }

        from_value(tx.clone())
            .map_err(|e| Error::Serde(e, format!("Failed to parse transaction: {hash}")))
    }
}
