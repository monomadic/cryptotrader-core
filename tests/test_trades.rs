use crate::common::*;
use cryptotrader::models::*;

mod common;

// trade_fixture(trade_type: TradeType, price: f64, current_price: f64, qty: f64);

fn trade_fixture_1() -> Trade {
    trade_fixture(TradeType::Buy, 0.00000689, 0.00000694, 58023.0)
}

fn trade_fixture_2() -> Trade {
    trade_fixture(TradeType::Buy, 1.0, 2.0, 2.0)
}

fn trade_fixture_3() -> Trade {
    trade_fixture(TradeType::Buy, 2.0, 1.0, 1.0)
}

fn trade_fixture_4() -> Trade {
    trade_fixture(TradeType::Sell, 2.0, 1.0, 1.0)
}

#[test]
fn trade_current_profit() {
    assert_eq!(trade_fixture_1().current_profit(), 0.0029011499999999635);
    assert_eq!(trade_fixture_2().current_profit(), 2.0);
    assert_eq!(trade_fixture_3().current_profit(), -1.0);
    assert_eq!(trade_fixture_4().current_profit(), -1.0);
}

#[test]
fn trade_current_profit_as_percent() {
    assert_eq!(trade_fixture_1().current_profit_as_percent(), 0.725689404934684);
    assert_eq!(trade_fixture_2().current_profit_as_percent(), 100.0);
    assert_eq!(trade_fixture_3().current_profit_as_percent(), -50.0);
    assert_eq!(trade_fixture_4().current_profit_as_percent(), -50.0);
}

#[test]
fn trade_works_with_one_entry() {
    let trade = trade_fixture(TradeType::Buy, 100.0, 150.0, 5.0);
    assert_eq!(trade.cost(), 500.0);
    assert_eq!(trade.value(), 750.0);
    assert_eq!(trade.profit(), 250.0);
    assert_eq!(trade.profit_as_percent(), 50.0);
    assert_eq!(sum_qty(vec![trade]), 5.0);
}

#[test]
fn trade_works_with_two_entries() {
    let trades = vec![
        trade_fixture(TradeType::Buy, 100.0, 150.0, 5.0),
        trade_fixture(TradeType::Buy, 200.0, 150.0, 10.0),
    ];
    assert_eq!(sum_qty(trades.clone()), 15.0);
    assert_eq!(sum_cost(trades.clone()), 2500.0);
    assert_eq!(average_cost(trades.clone()) as i32, 166);
}

#[test]
fn trade_group_trades_by_price() {
    let trade_group = group_trades_by_price(vec![
        trade_fixture(TradeType::Buy, 100.0, 150.0, 5.0),
        trade_fixture(TradeType::Buy, 100.0, 150.0, 5.0),
        trade_fixture(TradeType::Buy, 200.0, 150.0, 10.0),
    ]);

    assert_eq!(trade_group.len(), 2);
}

#[test]
fn trade_group_trades_by_trade_type() {
    let trade_groups = group_trades_by_trade_type(vec![
        trade_fixture(TradeType::Buy, 100.0, 150.0, 5.0),
        trade_fixture(TradeType::Buy, 100.0, 150.0, 5.0),
        trade_fixture(TradeType::Buy, 200.0, 150.0, 10.0),
        trade_fixture(TradeType::Sell, 200.0, 150.0, 10.0),
        trade_fixture(TradeType::Sell, 200.0, 150.0, 10.0),
        trade_fixture(TradeType::Buy, 200.0, 150.0, 10.0),
        trade_fixture(TradeType::Sell, 200.0, 150.0, 10.0),
    ]);

    assert_eq!(trade_groups.len(), 4);
}

#[test]
fn trade_average_trades() {
    let trade = average_trades(vec![
        trade_fixture(TradeType::Buy, 100.0, 150.0, 5.0),
        trade_fixture(TradeType::Buy, 100.0, 150.0, 5.0),
        trade_fixture(TradeType::Buy, 400.0, 150.0, 10.0),
    ]);

    assert_eq!(trade.trade_type, TradeType::Buy);
    assert_eq!(trade.price, 200.0);
    assert_eq!(trade.current_price(), 150.0);
}
