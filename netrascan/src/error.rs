use crate::ingest;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    EmptyTransaction(String),
    IngestError(ingest::Error),
    ParsingError(serde_json::Error),
    IoError(std::io::Error, String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyTransaction(wallet) => {
                write!(f, "ℹ️ Wallet {wallet} has no token transfer history.")
            }
            Self::IngestError(err) => write!(f, "{err}"),
            Self::ParsingError(err) => write!(f, "{err}"),
            Self::IoError(err, context) => {
                if context.is_empty() {
                    write!(f, "{err}")
                } else {
                    write!(f, "{context}: {err}")
                }
            }
        }
    }
}

impl Error {
    pub fn from_io(value: std::io::Error, context: impl Into<String>) -> Self {
        Self::IoError(value, context.into())
    }
}

impl From<ingest::Error> for Error {
    fn from(value: ingest::Error) -> Self {
        Self::IngestError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::ParsingError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::from_io(value, "")
    }
}
