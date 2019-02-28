pub mod binance;
// pub mod bittrex;
// pub mod bitfinex;
// pub mod kucoin;

use serde_derive::Deserialize;

use crate::error::*;
use crate::models::*;

pub trait ExchangeAPI {
    fn display(&self) -> String;
    fn btc_symbol(&self) -> String;
    fn usd_symbol(&self) -> String;
    fn base_pairs(&self) -> Vec<String>;
    fn btc_price(&self) -> CoreResult<Pair>;
    fn balances(&self) -> CoreResult<Vec<Asset>>;
    fn pair(&self, pair: &str) -> CoreResult<Pair>;
    fn all_pairs(&self) -> CoreResult<Vec<Pair>>;
    fn limit_buy(&self, symbol: &str, amount: f64, price: f64) -> CoreResult<()>;
    fn limit_sell(&self, symbol: &str, amount: f64, price: f64) -> CoreResult<()>;
    fn stop_loss(
        &self,
        symbol: &str,
        amount: f64,
        stop_price: f64,
        limit_price: f64,
    ) -> CoreResult<()>;
    fn open_orders(&self, pairs: Vec<Pair>) -> CoreResult<Vec<Order>>;
    fn past_orders(&self) -> CoreResult<Vec<Order>>;

    // TODO remove
    fn trades_for_pair(&self, pair: Pair) -> CoreResult<Vec<Trade>>;

    fn chart_data(&self, pair: &str, interval: &str) -> CoreResult<Vec<Candlestick>>;

    fn pair_format(&self, pair: Pair) -> String;
    fn string_to_pair(&self, pair: String, price: f64) -> Option<Pair>;

    fn btc_pair(&self, pairs: Vec<Pair>) -> Option<Pair> {
        find_pair_by_symbol_and_base(&self.btc_symbol(), &self.usd_symbol(), pairs)
    }

    fn symbol_and_base_to_pair_format(&self, symbol: &str, base: &str) -> String;

    fn usd_pair(&self, pairs: Vec<Pair>) -> Option<Pair> {
        find_pair_by_symbol_and_base(&self.usd_symbol(), &self.btc_symbol(), pairs)
    }

    // TODO: introduce client caching for pairs
    fn fiat_pair_for(&self, symbol: &str, pairs: Vec<Pair>) -> Option<Pair> {
        find_pair_by_symbol_and_base(symbol, &self.usd_symbol(), pairs)
        // if Some(pair) = find_pair_by_symbol_and_base(&self.usd_symbol(), &self.btc_symbol(), pairs) {
        //     pair
        // } else {
        //     find_pair_by_symbol_and_base(&self.usd_symbol(), &self.btc_symbol(), pairs)
        // }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum Exchange {
    Unknown,
    Bittrex,
    Binance,
    Kucoin,
}

impl std::str::FromStr for Exchange {
    type Err = ();

    fn from_str(s: &str) -> Result<Exchange, ()> {
        match s {
            "unknown" => Ok(Exchange::Unknown),
            "-" => Ok(Exchange::Unknown),
            "bittrex" => Ok(Exchange::Bittrex),
            "binance" => Ok(Exchange::Binance),
            "kucoin" => Ok(Exchange::Binance),
            _ => Err(()),
        }
    }
}

use std::string::ToString;
impl ToString for Exchange {
    fn to_string(&self) -> String {
        match self {
            &Exchange::Bittrex => "bittrex".into(),
            &Exchange::Binance => "binance".into(),
            &Exchange::Kucoin => "kucoin".into(),
            _ => "-".into(),
        }
    }
}
