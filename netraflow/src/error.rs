pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Parsing(serde_json::Error),
    Io(std::io::Error, String),
    Scanner(netrascan::error::Error),
    EtheriumClient(netrascan::ingest::Error),
    HeuristicsNotFound(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parsing(err) => write!(f, "{err}"),
            Self::Io(err, context) => {
                if context.is_empty() {
                    write!(f, "{err}")
                } else {
                    write!(f, "{context}: {err}")
                }
            }
            Self::Scanner(err) => write!(f, "{err}"),
            Self::EtheriumClient(err) => write!(f, "{err}"),
            Self::HeuristicsNotFound(path) => write!(f, "Heuristic file not found: {path}"),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Parsing(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value, "".to_string())
    }
}

impl From<netrascan::error::Error> for Error {
    fn from(value: netrascan::error::Error) -> Self {
        Self::Scanner(value)
    }
}

impl From<netrascan::ingest::Error> for Error {
    fn from(value: netrascan::ingest::Error) -> Self {
        Self::EtheriumClient(value)
    }
}
