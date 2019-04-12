use crate::models::*;
// use crate::utils::*;

#[derive(Debug, Clone)]
pub struct TradePresenter {
    pub trade: Trade,
    // pub trades: Vec<Vec<Trade>>,
    pub fiat_pair: Option<Pair>,
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
}
