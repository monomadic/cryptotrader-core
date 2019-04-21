use crate::models::*;
use crate::utils::price_percent;
// use crate::utils::*;

#[derive(Debug, Clone)]
pub struct TradePresenter {
    pub trade: Trade,
    // pub trades: Vec<Vec<Trade>>,
    pub fiat_pair: Pair,
//    prices: Vec<Price>,
    pub price_in_trade_currency: f64,
}

impl TradePresenter {
    // pub fn current_value_in_fiat(&self) -> Option<f64> {

    // }

    // pub fn current_cost_in_fiat(&self) -> Option<f64> {
    //     // let fiat_pair = self.fiat_pair.clone().expect(&format!("fiat pair to be there for {}", self.trade.pair.symbol));
    //     // Some(self.trade.current_cost() * fiat_pair.price)

    //     self.fiat_pair
    //         .clone()
    //         .map(|fiat| self.trade.value() * fiat.price)
    // }

    // pub fn current_profit_in_fiat(&self) -> Option<f64> {
    //     match self.trade.pair.base_type() {
    //         AssetType::Fiat => Some(self.trade.current_profit()),
    //         _ => self
    //             .fiat_pair
    //             .clone()
    //             .map(|fiat_pair| self.trade.current_profit() * fiat_pair.price),
    //     }
    // }

    // pub fn trade_groups(&self) -> Vec<Vec<Trade>> {
    //     self
    // }

//    pub fn current_price_in(&self, base: &str) -> Option<f64> {
//        self.prices.price_for(Pair{
//            symbol: self.trade.pair.symbol,
//            base:
//        })
//    }

    // the current price in the currency the trade was made in, eg if the trade was in btc,
    // this returns the current price in btc.
    pub fn current_price(&self) -> f64 {
        self.price_in_trade_currency
    }

    pub fn profit_as_percent(&self) -> f64 {
        price_percent(self.trade.sale_price, self.current_price())
    }

    pub fn current_cost_in_fiat(&self) -> f64 {
        0.0
    }
}
