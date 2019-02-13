#![allow(dead_code)]
use chrono::prelude::DateTime;
use chrono::Local;
use cryptotrader::models::*;
use std::time::UNIX_EPOCH;

pub static DEFAULT_SYMBOL_1: &str = "SMOSH";

pub fn trade_fixture(trade_type: TradeType, price: f64, current_price: f64, qty: f64) -> Trade {
    Trade {
        id: "id".to_string(),
        pair: Pair {
            symbol: DEFAULT_SYMBOL_1.to_string(),
            base: "BASE".to_string(),
            price: current_price,
        },
        trade_type,
        price,
        qty,
        time: DateTime::<Local>::from(UNIX_EPOCH),
        fee: 0.1,
        fee_symbol: None,
    }
}

pub fn order_fixture(order_type: TradeType, qty: f64, price: f64) -> Order {
    Order {
        id: "".to_string(),
        symbol: DEFAULT_SYMBOL_1.to_string(),
        order_type,
        qty,
        price,
    }
}

pub fn pair_fixture(base: String, symbol: String, price: f64) -> Pair {
    Pair {
        base,
        symbol,
        price,
    }
}

pub fn base_pair_fixture(base: &str, price: f64) -> Pair {
    pair_fixture(base.to_string(), DEFAULT_SYMBOL_1.to_string(), price)
}

pub fn btc_pair_fixture(price: f64) -> Pair {
    base_pair_fixture("BTC", price)
}

pub fn usd_pair_fixture(price: f64) -> Pair {
    base_pair_fixture("USD", price)
}
