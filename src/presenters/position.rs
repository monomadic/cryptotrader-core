use models::*;
use presenters::*;

#[derive(Debug, Clone)]
pub struct PositionPresenter {
    pub position:           Position,
    pub current_price:      f64,
    pub btc_price_in_usd:   f64,
}

impl PositionPresenter {
    pub fn order_presenters(&self) -> Vec<OrderPresenter> {
        self.position.orders.clone().into_iter().map(|o|
            OrderPresenter { order: o, btc_value: self.btc_price_in_usd } ).collect()
    }

    pub fn current_value_in_btc(&self) -> f64 {
        self.position.remaining_qty() * self.current_price
    }

    pub fn current_value_in_usd(&self) -> f64 {
        self.current_value_in_btc() * self.btc_price_in_usd
    }

    pub fn percent_change(&self) -> f64 {
        price_percent(self.position.entry_price(), self.current_price)
    }

    /// price of remaining units at the current price - those units at buy price
    pub fn unrealised_profit_btc(&self) -> f64 {
        ((self.current_price * self.position.remaining_qty()) - (self.position.entry_price() * self.position.remaining_qty()))
    }

    pub fn unrealised_profit_usd(&self) -> f64 {
        self.unrealised_profit_btc() * self.btc_price_in_usd
    }

    pub fn realised_profit_btc(&self) -> f64 {
        if let Some(exit_price) = self.position.exit_price() {
            ((exit_price * self.position.sell_qty()) - (self.position.entry_price() * self.position.sell_qty()))
        } else { 0.0 }
    }

    pub fn realised_profit_usd(&self) -> f64 {
        self.realised_profit_btc() * self.btc_price_in_usd
    }

    pub fn realised_profit_percent(&self) -> f64 {
        if let Some(exit_price) = self.position.exit_price() {
            price_percent(self.position.entry_price(), exit_price)
        } else { 0.0 }
    }
}

pub fn price_percent(entry_price: f64, exit_price: f64) -> f64 {
    if entry_price < exit_price { (100. / entry_price * exit_price) - 100. }
    else { -(100. + -100. / entry_price * exit_price) }
}

pub fn total_btc_staked(presenters: Vec<PositionPresenter>) -> f64 {
    presenters.into_iter().map(|a| a.current_value_in_btc()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn order_fixture(order_type: TradeType, qty: f64, price: f64) -> Order {
        Order{ id: "".to_string(), symbol: "".to_string(), order_type: order_type, qty: qty, price: price }
    }

    #[test]
    fn test_position_presenter_state_partial_1() {
        let position = Position::new(vec![
            order_fixture(TradeType::Buy, 2.0, 100.0), // value: 200
            order_fixture(TradeType::Sell, 1.0, 100.0), // sold: 100 worth, remaining: 100, profit: 0
        ]);

        let presenter = PositionPresenter {
            position:           position.first().unwrap().clone(),
            current_price:      110.0,
            btc_price_in_usd:   2.0,
        };

        assert_eq!(presenter.current_value_in_btc(), 110.0);
        assert_eq!(presenter.current_value_in_usd(), 220.0);
        assert_eq!(presenter.percent_change(), 10.0);
        assert_eq!(presenter.position.buy_qty(), 2.0);
        assert_eq!(presenter.position.sell_qty(), 1.0);
        assert_eq!(presenter.position.remaining_qty(), 1.0);
        assert_eq!(presenter.position.entry_price(), 100.0);
        assert_eq!(presenter.position.exit_price(), Some(100.0));
        assert_eq!(presenter.realised_profit_btc(), 0.0);
        assert_eq!(presenter.unrealised_profit_btc(), 10.0);
    }

    #[test]
    fn test_position_presenter_state_partial_2() {
        let position = Position::new(vec![
            order_fixture(TradeType::Buy, 2.0, 100.0), // value: 200
            order_fixture(TradeType::Buy, 2.0, 110.0), // value: 4x105=420, qty: 4
            order_fixture(TradeType::Sell, 1.0, 150.0), // sold: 1, qty: 3
        ]);

        let presenter = PositionPresenter {
            position:           position.first().unwrap().clone(),
            current_price:      110.0,
            btc_price_in_usd:   2.0,
        };

        assert_eq!(presenter.position.buy_qty(), 4.0);
        assert_eq!(presenter.position.sell_qty(), 1.0);
        assert_eq!(presenter.position.remaining_qty(), 3.0);
        assert_eq!(presenter.position.entry_price(), 105.0);
        assert_eq!(presenter.position.exit_price(), Some(150.0));
        assert_eq!(presenter.current_value_in_btc(), 330.0);
        assert_eq!(presenter.current_value_in_usd(), 660.0);
        assert_eq!(presenter.realised_profit_btc().floor(), 45.0); // current btc value of profit
        assert_eq!(presenter.unrealised_profit_btc(), 15.0); // 330 possible sale, paid 3x105=315 = 15
        assert_eq!(presenter.percent_change().floor(), 4.0);
    }
}