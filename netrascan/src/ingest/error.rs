pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Rpc(i64, String),
    Reqwest(reqwest::Error),
    Serde(serde_json::Error, String),
    ParseInt(std::num::ParseIntError),
    EmptyBlockNumber,
    EmptyBlockResult,
    TxNotFound(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rpc(code, msg) => write!(f, " RPC error (code {code}): {msg}"),
            Self::Reqwest(err) => write!(f, "Request error: {err}"),
            Self::Serde(err, context) => write!(f, "{context}: {err}"),
            Self::ParseInt(err) => write!(f, "Parsing int error: {err}"),
            Self::EmptyBlockNumber => write!(
                f,
                "Received empty or missing block number from RPC response"
            ),
            Self::EmptyBlockResult => write!(f, "Failed to get block data"),
            Self::TxNotFound(hash) => write!(f, "Transaction not found: {hash}"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value, String::from(""))
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}
