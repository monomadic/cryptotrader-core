#![allow(dead_code)]
use cryptotrader::models::*;

pub static DEFAULT_SYMBOL_1: &str = "SMOSH";

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
