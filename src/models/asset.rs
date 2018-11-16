use exchanges::*;

#[derive(Debug, Clone)]
pub struct Asset {
    pub symbol: String,
    pub amount: f64,
    pub locked: f64,
    pub exchange: Exchange,
}

impl Default for Asset {
    fn default() -> Self {
        Asset {
            symbol: "<None>".to_string(),
            amount: 0.0,
            locked: 0.0,
            exchange: Exchange::Binance,
        }
    }
}
