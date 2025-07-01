use common::TxResponse;
use reqwest::Client;

use crate::model::EtherscanResponse;
use super::Result;

use super::TransactionClient;

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
}

impl TransactionClient for EtherscanClient {
    async fn fetch_token_transfer(
        &self, 
        wallet: &str, 
        page: usize, 
        offset: usize
    ) -> Result<Vec<TxResponse>> {
        let url = format!(
            "https://api.etherscan.io/api?module=account&action=tokentx&address={wallet}&page={page}&offset={offset}&sort=desc&apikey={}",
            self.api_key
        );

        let response = self.client.get(&url).send().await?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            eprintln!("❌ Error parsing response: {text}");
        }

        // let etherscan: EtherscanResponse = response.json().await?;
        let etherscan: EtherscanResponse = match serde_json::from_str(&text) {
            Ok(response) => response,
            Err(err) => {
                eprintln!("❌ Error parsing response: {text}");
                // eprintln!("❌ Failed to parse response to EtherscanResponse");
                return Err(err.into());
            }
        };
        Ok(etherscan.result)
    }

    async fn fetch_all_token_transfer(
        &self,
        wallet: &str,
        offset: usize,
    ) -> Result<Vec<TxResponse>> {
        let mut page = 1;
        let mut all_txs: Vec<TxResponse> = Vec::new();

        loop {
            let txs = self.fetch_token_transfer(wallet, page, offset).await?;

            if txs.is_empty() {
                break;
            }

            println!("📦 Fetched page {page} with {} txs", txs.len());

            all_txs.extend(txs);
            page += 1;
        }

        Ok(all_txs)
    }
}
