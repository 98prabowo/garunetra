pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    ParsingError(serde_json::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestError(err) => write!(f, "{err}"),
            Self::ParsingError(err) => write!(f, "{err}"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::RequestError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::ParsingError(value)
    }
}
