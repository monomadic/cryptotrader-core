pub mod binance;
pub mod huobi;
use crate::error::*;
use crate::models::*;
use serde_derive::Deserialize;

pub trait ExchangeAPI {
    fn display(&self) -> String;
    fn btc_symbol(&self) -> String;
    fn usd_symbol(&self) -> String;
    fn base_pairs(&self) -> Vec<String>;
    fn balances(&self) -> CoreResult<Vec<Asset>>;
    fn pair(&self, pair: &str) -> CoreResult<Pair>;
    fn all_pairs(&self) -> CoreResult<Vec<Pair>>;
    fn all_prices(&self) -> CoreResult<Vec<Price>>;
    fn limit_buy(&self, symbol: &str, amount: f64, price: f64) -> CoreResult<()>;
    fn limit_sell(&self, symbol: &str, amount: f64, price: f64) -> CoreResult<()>;
    fn open_orders(&self) -> CoreResult<Vec<Order>>;
    fn past_orders(&self) -> CoreResult<Vec<Order>>;
    fn book_tickers(&self) -> CoreResult<Vec<BookTicker>>;
    fn trades_for_pair(&self, pair: Pair) -> CoreResult<Vec<Trade>>;
    fn chart_data(&self, pair: &str, interval: &str) -> CoreResult<Vec<Candlestick>>;
    fn market_depth(&self, pair: &str) -> CoreResult<Depth>;
    fn symbol_and_base_to_pair_format(&self, symbol: &str, base: &str) -> String;
    fn stop_loss(
        &self,
        symbol: &str,
        amount: f64,
        stop_price: f64,
        limit_price: f64,
    ) -> CoreResult<()>;

    // default implementations

    fn btc_usd_pair(&self) -> Pair {
        // find_pair_by_symbol_and_base(&self.btc_symbol(), &self.usd_symbol(), pairs)
        Pair {
            symbol: self.btc_symbol(),
            base: self.usd_symbol(),
        }
    }

    fn btc_price(&self, prices: Vec<Price>) -> Price {
        Price::find_first_btc_usd_price(&prices).expect("btc price not found") // fix to be exchange specific
    }

    /// find all trades for a symbol across all base pairs.
    fn trades_for_pairs(&self, pairs: Vec<Pair>) -> CoreResult<Vec<Trade>> {
        let mut trades = Vec::new();

        for pair in pairs {
            let mut result = self.trades_for_pair(pair)?;
            trades.append(&mut result);
        }

        // sort by time
        trades.sort_by(|a, b| a.time.cmp(&b.time));
        Ok(trades)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum Exchange {
    Unknown,
    Binance,
    Huobi,
    Kucoin,
}

impl std::str::FromStr for Exchange {
    type Err = ();

    fn from_str(s: &str) -> Result<Exchange, ()> {
        match s {
            "unknown" => Ok(Exchange::Unknown),
            "-" => Ok(Exchange::Unknown),
            "huobi" => Ok(Exchange::Huobi),
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
            &Exchange::Binance => "binance".into(),
            &Exchange::Huobi => "huobi".into(),
            &Exchange::Kucoin => "kucoin".into(),
            _ => "-".into(),
        }
    }
}
