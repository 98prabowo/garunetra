mod error;
mod etherium_client;
mod etherscan_client;

use async_trait::async_trait;
use common::model::TxResponse;

pub use error::{Error, Result};
pub use etherium_client::EtheriumClient;
pub use etherscan_client::EtherscanClient;

#[async_trait]
pub trait TransactionClient {
    async fn fetch_token_transfer(
        &self,
        wallet: &str,
        start_block: u64,
        offset: usize,
    ) -> Result<Vec<TxResponse>>;

    async fn fetch_all_token_transfer(
        &self,
        wallet: &str,
        start_block: u64,
        offset: usize,
    ) -> Result<Vec<TxResponse>>;
}
