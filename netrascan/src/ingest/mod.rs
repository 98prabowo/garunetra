mod etherscan_client;
mod error;

use common::TxResponse;

pub use etherscan_client::EtherscanClient;
pub use error::{Error, Result};

pub trait TransactionClient {
    async fn fetch_token_transfer(
        &self,
        wallet: &str,
        page: usize,
        offset: usize,
    ) -> Result<Vec<TxResponse>>;

    async fn fetch_all_token_transfer(
        &self,
        wallet: &str,
        offset: usize,
    ) -> Result<Vec<TxResponse>>;
}
