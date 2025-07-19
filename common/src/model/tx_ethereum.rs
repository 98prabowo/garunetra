use serde::Deserialize;

use crate::utils::Heuristics;

use super::TxCategory;

#[derive(Debug, Deserialize)]
pub struct TxEthereum {
    pub hash: String,
    pub from: String,
    pub to: Option<String>,
    pub input: String,
    pub value: String,
}

impl TxEthereum {
    pub fn categorize(&self, heuristics: &Heuristics) -> TxCategory {
        self.to
            .as_ref()
            .map(|addr| addr.to_ascii_lowercase())
            .map(|to| {
                if heuristics.is_known_cex(&to) {
                    TxCategory::Foreign
                } else if heuristics.is_known_bridge(&to) {
                    TxCategory::Bridge
                } else if heuristics.is_known_domestic(&self.from, &to) {
                    TxCategory::Domestic
                } else {
                    TxCategory::Foreign
                }
            })
            .unwrap_or(TxCategory::Unknown)
    }
}
