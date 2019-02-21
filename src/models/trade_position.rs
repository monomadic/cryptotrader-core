use crate::models::*;

#[derive(Debug, Clone)]
pub struct TradePosition {
    pub trade: Trade, // for purchase cost (profit) and qty bought (validity)
    // pub asset: Asset,
    pub pairs: Vec<Pair>,
}

#[derive(Debug, Clone)]
pub enum PositionStatus {
    Open,
    Closed,
}

impl TradePosition {
    pub fn size(&self) -> f64 {
        self.asset.amount
    }

    pub fn profit(&self) -> f64 {
        self.trade.cost()
    }
}
