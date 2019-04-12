use crate::models::*;

#[derive(Debug, Clone)]
pub struct OrderPresenter {
    pub order: Order,
    pub prices: Vec<Price>,
}

impl OrderPresenter {
    pub fn current_value_in_btc(&self) -> f64 {
        self.order.qty
            * Price::find_first_btc_price_for_symbol(&self.order.pair.symbol, self.prices.clone())
                .map_or(0.0, |p| p.price)
    }

    //    pub fn current_value_in_usd(&self) -> f64 {
    //        self.order.qty
    //            * Price::find_first_btc_price_for_symbol(&self.order.pair.symbol, self.prices)
    //            .map_or(0.0, |p| p.price)
    //    }
}
