pub type PriceLookupFn = fn(&str, i64) -> Option<f64>;

pub fn dummy_price_lookup(_symbol: &str, _timestamp: i64) -> Option<f64> {
    Some(100.0)
}
