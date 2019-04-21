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

use std::{fmt, fmt::Display};
impl Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

pub trait AssetExtensions {
    fn filter_zero_balances(&self) -> Self;
    fn filter_small_balances(&self, min: f64) -> Self;
}

impl AssetExtensions for Vec<Asset> {
    fn filter_zero_balances(&self) -> Self {
        self.filter_small_balances(0.0)
    }

    fn filter_small_balances(&self, min: f64) -> Self {
        self.into_iter()
            .filter(|asset| asset.amount > min)
            .map(|a| a.clone())
            .collect()
    }
}
