use models::*;
// use presenters::*;

#[derive(Debug, Clone)]
pub struct OrderPresenter {
    pub order:  Order,
    pub btc_value:  f64,
}

impl OrderPresenter {
    pub fn current_value_in_btc(&self) -> f64 {
        self.order.qty * self.order.price
    }
    
    pub fn current_value_in_usd(&self) -> f64 {
        self.order.qty * self.order.price * self.btc_value
    }
}
