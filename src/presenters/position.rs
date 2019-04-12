//use crate::models::*;
//use crate::utils::*;
//use chrono::{offset::Local, prelude::DateTime};
//
//#[derive(Debug, Clone)]
//pub struct PositionPresenter {
//    pub position: Position,
//    pub prices: Vec<Price>,
//    pub btc_price_in_usd: f64,
//}
//
//impl PositionPresenter {
//    //    pub fn price_in(&self, symbol: &str) -> Option<f64> {
//    //        Price::
//    //        find_pair_by_symbol_and_base(&self.symbol(), symbol, self.prices.clone()).map(|p| p.price)
//    //    }
//
//    //    pub fn value_in(&self, symbol: &str) -> Option<f64> {
//    //        if let Some(price) = self.price_in(symbol) {
//    //            Some(price * self.qty())
//    //        } else {
//    //            None
//    //        }
//    //    }
//
//    pub fn time(&self) -> Option<DateTime<Local>> {
//        self.position.trades.first().map(|t| t.time)
//    }
//
//    pub fn qty(&self) -> f64 {
//        self.position.qty()
//    }
//
//    pub fn current_price_in_btc(&self) -> f64 {
//        find_pair_by_symbol_and_base(&self.symbol(), "BTC", self.pairs.clone())
//            .map(|p| p.price)
//            .unwrap_or(0.0)
//    }
//
//    // note: maybe discriminate based on trade types...
//    pub fn current_price_in_usd(&self) -> f64 {
//        find_pair_by_symbol_and_base(&self.symbol(), "USDT", self.pairs.clone())
//            .map(|p| p.price)
//            .unwrap_or(self.current_price_in_btc() * self.btc_price_in_usd)
//    }
//
//    pub fn symbol(&self) -> String {
//        self.position.symbol()
//    }
//
//    pub fn is_valid(&self) -> bool {
//        self.position.remaining_qty() == self.position.asset.amount
//    }
//
//    // pub fn order_presenters(&self) -> Vec<OrderPresenter> {
//    //     self.position
//    //         .trades
//    //         .clone()
//    //         .into_iter()
//    //         .map(|t| OrderPresenter {
//    //             order: t,
//    //             btc_value: 55.0,
//    //         })
//    //         .collect()
//    // }
//
//    // pub fn pair_positions(&self) -> Vec<PairPosition> {}
//
//    // delete
//    pub fn current_value_in_btc(&self) -> f64 {
//        self.qty() * self.current_price_in_btc()
//    }
//
//    // delete
//    pub fn current_value_in_usd(&self) -> f64 {
//        self.current_value_in_btc() * self.btc_price_in_usd
//    }
//
//    pub fn entry_price_in_btc(&self) -> f64 {
//        //fixme: tech debt
//        match self.position.base_type().unwrap_or(AssetType::Bitcoin) {
//            AssetType::Bitcoin => self.position.entry_price(),
//            _ => self.position.entry_price() / self.btc_price_in_usd,
//        }
//    }
//
//    pub fn entry_price_in_usd(&self) -> f64 {
//        //fixme: tech debt
//        match self.position.base_type().unwrap_or(AssetType::Bitcoin) {
//            AssetType::Fiat => self.position.entry_price(),
//            _ => self.position.entry_price() * self.btc_price_in_usd,
//        }
//    }
//
//    pub fn percent_change(&self) -> f64 {
//        price_percent(self.entry_price_in_btc(), self.current_price_in_btc())
//    }
//
//    /// price of remaining units at the current price - those units at buy price
//    pub fn unrealised_profit_btc(&self) -> f64 {
//        (self.current_price_in_btc() - self.entry_price_in_btc()) * self.position.remaining_qty()
//    }
//
//    pub fn unrealised_profit_usd(&self) -> f64 {
//        (self.current_price_in_usd() * self.position.remaining_qty())
//            - (self.entry_price_in_usd() * self.position.remaining_qty())
//    }
//
//    pub fn realised_profit_btc(&self) -> f64 {
//        if let Some(exit_price) = self.position.exit_price() {
//            ((exit_price * self.position.sell_qty())
//                - (self.position.entry_price() * self.position.sell_qty()))
//        } else {
//            0.0
//        }
//    }
//
//    pub fn realised_profit_usd(&self) -> f64 {
//        self.realised_profit_btc() * self.current_price_in_usd() // WRONG
//    }
//
//    pub fn realised_profit_percent(&self) -> f64 {
//        if let Some(exit_price) = self.position.exit_price() {
//            price_percent(self.position.entry_price(), exit_price)
//        } else {
//            0.0
//        }
//    }
//}
//
//// delete
//pub fn total_btc_staked(presenters: Vec<PositionPresenter>) -> f64 {
//    presenters
//        .into_iter()
//        .map(|a| a.current_value_in_btc())
//        .sum()
//}
