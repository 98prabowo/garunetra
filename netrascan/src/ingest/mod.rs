mod etherscan_client;

use common::TxResponse;

pub use etherscan_client::EtherscanClient;

pub trait TransactionClient {
    async fn fetch_token_transfer(&self, wallet: &str) -> Result<Vec<TxResponse>, reqwest::Error>;
}
