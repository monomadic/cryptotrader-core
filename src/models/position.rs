use crate::error::*;
use crate::models::*;
use crate::utils::*;

#[derive(Debug, Clone)]
pub struct Position {
    pub trades: Vec<Trade>,
    pub asset: Asset,
}

impl Position {
    pub fn new(trades: Vec<Trade>, asset: Asset) -> CoreResult<Self> {
        if trades.is_empty() {
            return Err(Box::new(TrailerError::Generic(format!(
                "cannot create a position for {} without trades.",
                asset
            ))));
        };

        Ok(Position { trades, asset })
    }

    pub fn symbol(&self) -> String {
        self.trades
            .first()
            .map(|trade| trade.pair.symbol.clone())
            .expect("position does not contain trades.")
    }

    pub fn entry_price(&self) -> f64 {
        let entry_prices: f64 = self
            .buy_trades()
            .into_iter()
            .map(|o| o.sale_price * o.qty)
            .sum();
        let total_qty: f64 = self.buy_trades().into_iter().map(|o| o.qty).sum();

        entry_prices / total_qty
    }

    pub fn exit_price(&self) -> Option<f64> {
        if self.sell_trades().len() > 0 {
            Some(
                self.sell_trades()
                    .into_iter()
                    .map(|t| t.sale_price)
                    .sum::<f64>()
                    / self.sell_trades().len() as f64,
            )
        } else {
            None
        }
    }

    pub fn current_price(&self) -> f64 {
        0.0 // todo: fix to include most recent price
            //        self.buy_trades()
            //            .into_iter()
            //            .map(|o| o.pair.price)
            //            .sum::<f64>()
            //            / self.buy_trades().len() as f64
    }

    pub fn qty(&self) -> f64 {
        self.buy_qty() - self.sell_qty()
    }

    pub fn buy_qty(&self) -> f64 {
        self.buy_trades().into_iter().map(|o| o.qty).sum()
    }
    pub fn sell_qty(&self) -> f64 {
        self.sell_trades().into_iter().map(|o| o.qty).sum()
    }

    pub fn buy_cost(&self) -> f64 {
        self.entry_price() * self.buy_qty()
    }

    pub fn sell_cost(&self) -> f64 {
        self.exit_price().unwrap_or(0.0) * self.sell_qty()
    }

    // todo: memoize
    pub fn buy_trades(&self) -> Vec<Trade> {
        self.trades
            .clone()
            .into_iter()
            .filter(|t| t.trade_type == TradeType::Buy)
            .collect()
    }

    // todo: memoize
    pub fn sell_trades(&self) -> Vec<Trade> {
        self.trades
            .clone()
            .into_iter()
            .filter(|t| t.trade_type == TradeType::Sell)
            .collect()
    }

    /// averaged buy trade
    pub fn buy_trade(&self) -> Trade {
        average_trades(self.buy_trades())
    }

    /// averaged sell trade
    pub fn sell_trade(&self) -> Trade {
        average_trades(self.sell_trades())
    }

    pub fn remaining_qty(&self) -> f64 {
        // println!("remaining_qty: {}", self.asset.amount);
        self.asset.amount
        // self.buy_qty() - self.sell_qty()
    }

    pub fn current_profit_as_percent(&self) -> f64 {
        price_percent(self.entry_price(), self.current_price())
    }

    pub fn base_type(&self) -> Option<AssetType> {
        self.trades.first().map(|t| t.pair.base_type())
    }
}
