use common::WalletType;

pub fn classify_wallet(score: f32) -> WalletType {
    match score {
        s if s >= 0.6 => WalletType::Domestic,
        s if s > 0.2 && s < 0.6 => WalletType::Bridge,
        s if s > -0.5 && s <= 0.2 => WalletType::Mixer,
        s if s <= -0.5 => WalletType::Foreign,
        _ => WalletType::Unknown,
    }
}
