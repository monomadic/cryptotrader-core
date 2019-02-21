use crate::exchanges::*;
use crate::models::pair::AssetType;

#[derive(Debug, Clone)]
pub struct Asset {
    pub symbol: String,
    pub amount: f64,
    pub locked: f64,
    pub exchange: Exchange,
}

impl Asset {
    pub fn asset_type(&self) -> AssetType {
        AssetType::from_symbol(&self.symbol)
    }
}
