//#![allow(dead_code)]
//use chrono::prelude::DateTime;
//use chrono::Local;
//use cryptotrader::models::*;
//use std::time::UNIX_EPOCH;
//
//pub static DEFAULT_SYMBOL: &str = "SMOSH";
//pub static DEFAULT_FIAT: &str = "FIAT";
//
//pub static DEFAULT_SYMBOL_1: &str = "SMOSH";
//
//pub fn trade_fixture(
//    trade_type: TradeType,
//    sale_price: f64,
//    current_price: f64,
//    qty: f64,
//) -> Trade {
//    Trade {
//        id: "id".to_string(),
//        pair: Pair {
//            symbol: DEFAULT_SYMBOL_1.to_string(),
//            base: "BASE".to_string(),
//        },
//        trade_type,
//        sale_price,
//        qty,
//        time: DateTime::<Local>::from(UNIX_EPOCH),
//        fee: 0.1,
//        fee_symbol: None,
//    }
//}
//
//pub fn order_fixture(order_type: TradeType, qty: f64, purchase_price: f64) -> Order {
//    Order {
//        id: "".to_string(),
//        pair: default_pair(),
//        order_type,
//        qty,
//        purchase_price,
//    }
//}
//
//pub fn pair_fixture(base: String, symbol: String) -> Pair {
//    Pair { base, symbol }
//}
//
//pub fn default_pair() -> Pair {
//    Pair {
//        base: DEFAULT_FIAT.to_string(),
//        symbol: DEFAULT_SYMBOL_1.to_string(),
//    }
//}
//
//pub fn base_pair_fixture(base: &str) -> Pair {
//    pair_fixture(base.to_string(), DEFAULT_SYMBOL_1.to_string())
//}
//
//pub fn btc_pair_fixture() -> Pair {
//    base_pair_fixture("BTC")
//}
//
//pub fn usd_pair_fixture() -> Pair {
//    base_pair_fixture("USD")
//}
