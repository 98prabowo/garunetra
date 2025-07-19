use common::model::TxResponse;
use reqwest::Client;
use serde::Deserialize;
use serde_json::from_str;

use crate::error::Result;

#[derive(Debug, Deserialize)]
pub struct EtherscanResponse {
    pub result: Vec<TxResponse>,
}

pub struct EtherscanClient {
    api_key: String,
    client: Client,
}

impl EtherscanClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: Client::new(),
        }
    }

    pub async fn fetch_token_transfer(
        &self,
        wallet: &str,
        start_block: u64,
        offset: usize,
    ) -> Result<Vec<TxResponse>> {
        let url = format!(
            "https://api.etherscan.io/api?module=account\
                &action=tokentx\
                &address={wallet}\
                &startblock={start_block}\
                &endblock=13000000\
                &sort=asc\
                &offset={offset}\
                &apikey={}",
            self.api_key
        );

        let response = self.client.get(&url).send().await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            eprintln!("❌ Error parsing response: {body}");
        }

        let etherscan: EtherscanResponse = from_str(&body)?;
        Ok(etherscan.result)
    }

    pub async fn fetch_all_token_transfer(
        &self,
        wallet: &str,
        mut start_block: u64,
        offset: usize,
    ) -> Result<Vec<TxResponse>> {
        let mut all_txs: Vec<TxResponse> = Vec::new();

        loop {
            let txs: Vec<TxResponse> = self
                .fetch_token_transfer(wallet, start_block, offset)
                .await?;

            if txs.is_empty() {
                break;
            }

            println!(
                "📦 Fetched {} txs starting from block {start_block}",
                txs.len()
            );

            start_block = txs
                .last()
                .and_then(|tx| tx.block_number.parse::<u64>().ok())
                .unwrap_or(start_block + 1);

            all_txs.extend(txs);
        }

        Ok(all_txs)
    }
}
