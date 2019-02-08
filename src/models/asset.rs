use crate::exchanges::*;

#[derive(Debug, Clone)]
pub struct Asset {
    pub symbol: String,
    pub amount: f64,
    pub locked: f64,
    pub exchange: Exchange,
}
