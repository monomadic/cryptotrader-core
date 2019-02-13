use crate::common::*;
use cryptotrader::models::*;

mod common;

// trade_fixture(trade_type: TradeType, price: f64, current_price: f64, qty: f64);

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
