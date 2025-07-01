use common::TxResponse;
use reqwest::Client;

use crate::model::EtherscanResponse;

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
    async fn fetch_token_transfer(&self, wallet: &str) -> Result<Vec<TxResponse>, reqwest::Error> {
        let url = format!(
            "https://api.etherscan.io/api?module=account&action=tokentx&address={wallet}&sort=desc&apikey={}",
            self.api_key
        );

        let response = self.client.get(&url).send().await?;
        let etherscan: EtherscanResponse = response.json().await?;
        Ok(etherscan.result)
    }
}
