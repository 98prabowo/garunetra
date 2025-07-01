use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WalletType {
    Domestic,
    Foreign,
    Bridge,
    Mixer,
    Unknown,
}
