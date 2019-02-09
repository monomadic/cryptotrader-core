use crate::common::*;
use cryptotrader::{models::*, presenters::*};

mod common;

#[test]
fn test_position_presenter_price_and_value() {
    let position: Vec<Position> = Position::new(vec![
        order_fixture(TradeType::Buy, 20.0, 100.0),  // value: 200
        order_fixture(TradeType::Sell, 10.0, 100.0), // sold: 100 worth, remaining: 100, profit: 0
    ]);

    let presenter = PositionPresenter {
        position: position.first().unwrap().clone(),
        pairs: vec![
            base_pair_fixture("BSD", 0.00010100),
            base_pair_fixture("USD", 100.0),
        ],
    };

    assert_eq!(presenter.price_in_btc(), 0.00010100);
    assert_eq!(presenter.price_in("BTC"), Some(0.00010100));
    assert_eq!(presenter.price_in("USD"), Some(100.0));

    assert_eq!(presenter.qty(), 10.0);
    assert_eq!(presenter.value_in("BTC"), Some(100.0));
    assert_eq!(presenter.value_in("USD"), Some(100.0));
}

#[test]
fn test_position_presenter_with_one_buy_one_sell() {
    let position: Vec<Position> = Position::new(vec![
        order_fixture(TradeType::Buy, 2.0, 100.0),  // value: 200
        order_fixture(TradeType::Sell, 1.0, 100.0), // sold: 100 worth, remaining: 100, profit: 0
    ]);

    let presenter = PositionPresenter {
        position: position.first().unwrap().clone(),
        pairs: vec![btc_pair_fixture(110.0), usd_pair_fixture(2.0)],
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
fn test_position_presenter_with_two_buys_one_sell() {
    let positions: Vec<Position> = Position::new(vec![
        order_fixture(TradeType::Buy, 2.0, 100.0),  // value: 200
        order_fixture(TradeType::Buy, 2.0, 110.0),  // value: 4x105=420, qty: 4
        order_fixture(TradeType::Sell, 1.0, 150.0), // sold: 1, qty: 3
    ]);
    let presenter = PositionPresenter {
        position: positions.first().unwrap().clone(),
        pairs: vec![usd_pair_fixture(110.0), btc_pair_fixture(2.0)],
    };
    assert_eq!(presenter.current_value_in_btc(), 330.0);
    assert_eq!(presenter.current_value_in_usd(), 660.0);
    assert_eq!(presenter.realised_profit_btc().floor(), 45.0); // current btc value of profit
    assert_eq!(presenter.unrealised_profit_btc(), 15.0); // 330 possible sale, paid 3x105=315 = 15
    assert_eq!(presenter.percent_change().floor(), 4.0);
}

#[test]
fn test_position_presenter_state_partial_3() {
    let positions: Vec<Position> = Position::new(vec![
        order_fixture(TradeType::Buy, 2.0, 100.0),  // value: 200
        order_fixture(TradeType::Buy, 2.0, 110.0),  // value: 420, qty: 4
        order_fixture(TradeType::Sell, 1.0, 150.0), // sold: 1, qty: 3
        order_fixture(TradeType::Buy, 2.0, 150.0),  // bought: 2, qty: 5
    ]);
    let presenter = PositionPresenter {
        position: positions.first().unwrap().clone(),
        pairs: vec![usd_pair_fixture(110.0), btc_pair_fixture(2.0)],
    };
    assert_eq!(presenter.current_value_in_btc(), 330.0);
    assert_eq!(presenter.current_value_in_usd(), 660.0);
    assert_eq!(presenter.realised_profit_btc().floor(), 45.0); // current btc value of profit
    assert_eq!(presenter.unrealised_profit_btc(), 15.0); // 330 possible sale, paid 3x105=315 = 15
    assert_eq!(presenter.percent_change().floor(), 4.0);
}
