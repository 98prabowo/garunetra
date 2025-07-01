use common::TxRecord;

pub fn calculate_score(txs: &[TxRecord]) -> f32 {
    let mut score = 0f32;

    for tx in txs {
        if tx.token_symbol == "IDRT" || tx.token_symbol == "BIDR" {
            score += 0.5;
        }

        if tx.to.contains("binance") || tx.to.contains("kucoin") || tx.to.contains("okx") {
            score -= 0.5;
        }

        // TODO: Add timezone heuristic if timestamp is decoded.
    }

    score
}
