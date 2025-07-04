use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TxCategory {
    Domestic,
    Bridge,
    Foreign,
    Unknown,
}

impl std::fmt::Display for TxCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TxCategory::Foreign => write!(f, "Foreign"),
            TxCategory::Bridge => write!(f, "Bridge"),
            TxCategory::Domestic => write!(f, "Domestice"),
            TxCategory::Unknown => write!(f, "Unknown"),
        }
    }
}
