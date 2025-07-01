use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct WalletAddress {
    pub address: String,
}

impl WalletAddress {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
        }
    }
}
