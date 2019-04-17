#![allow(dead_code)]
#![allow(unused_variables)]

use crate::utils::*;
use crate::{error::*, exchanges::*, models::*};
use huobi_api;
use log::info;

#[derive(Clone)]
pub struct HuobiAPI {
    client: huobi_api::Client,
    default_account: u32,
}

pub static BASE_PAIRS: [&str; 2] = ["USDT", "BTC"];
pub static BTC_SYMBOL: &str = "BTC";
pub static USD_SYMBOL: &str = "USDT";

impl HuobiAPI {
    pub fn new(api_key: &str, secret_key: &str) -> CoreResult<Self> {
        let client = huobi_api::Client::new(api_key, secret_key);
        let accounts: Vec<huobi_api::models::Account> = client.accounts()?;

        if let Some(account) = accounts.first() {
            return Ok(Self {
                client,
                default_account: account.account_id,
            });
        };

        Err(Box::new(TrailerError::APIError(
            "no accounts found.".to_string(),
        )))
    }
}

impl ExchangeAPI for HuobiAPI {
    fn display(&self) -> String {
        "huobi".to_string()
    }

    fn btc_symbol(&self) -> String {
        "btc".to_string()
    }

    fn usd_symbol(&self) -> String {
        "usdt".to_string()
    }

    fn base_pairs(&self) -> Vec<String> {
        BASE_PAIRS
            .into_iter()
            .map(|pair| pair.to_string())
            .collect()
    }

    fn balances(&self) -> CoreResult<Vec<Asset>> {
        Ok(self
            .client
            .balance(self.default_account)?
            .list
            .into_iter()
            .map(|a| Asset {
                symbol: a.currency.to_uppercase(),
                amount: a.balance,
                locked: 0.0,
                exchange: Exchange::Huobi,
            })
            .collect())
    }

    fn pair(&self, pair: &str) -> CoreResult<Pair> {
        unimplemented!()
    }

    fn all_pairs(&self) -> CoreResult<Vec<Pair>> {
        Ok(self
            .client
            .common_symbols()?
            .into_iter()
            .map(|p: huobi_api::Pair| Pair {
                base: p.base_currency,
                symbol: p.symbol,
            })
            .collect())
    }

    fn all_prices(&self) -> CoreResult<Vec<Price>> {
        Ok(self
            .client
            .tickers()?
            .into_iter()
            .map(|t: huobi_api::Ticker| {
                string_to_pair(&t.symbol.to_uppercase()).map(|pair| Price {
                    pair,
                    price: t.close,
                })
            })
            .filter_map(|e| e)
            .collect())
    }

    fn limit_buy(&self, symbol: &str, amount: f64, price: f64) -> CoreResult<()> {
        unimplemented!()
    }

    fn limit_sell(&self, symbol: &str, amount: f64, price: f64) -> CoreResult<()> {
        unimplemented!()
    }

    fn open_orders(&self) -> CoreResult<Vec<Order>> {
        unimplemented!()
    }

    fn past_orders(&self) -> CoreResult<Vec<Order>> {
        unimplemented!()
    }

    fn book_tickers(&self) -> CoreResult<Vec<BookTicker>> {
        unimplemented!()
    }

    fn trades_for_pair(&self, pair: Pair) -> CoreResult<Vec<Trade>> {
        Ok(self
            .client
            .orders(&pair_to_string(pair.clone()))?
            .into_iter()
            .map(|t| {
                //                let pair = string_to_pair(&t.symbol).expect("pair to be found");
                Trade {
                    fee: t.field_fees,
                    fee_symbol: Some(pair.clone().base),
                    id: t.id.to_string(),
                    pair: pair.clone(),
                    sale_price: t.price,
                    qty: t.amount,
                    time: local_datetime_from_unix(t.created_at as u64),
                    trade_type: string_to_order_type(&t.order_type),
                }
            })
            .collect())
    }

    fn chart_data(&self, pair: &str, interval: &str) -> CoreResult<Vec<Candlestick>> {
        unimplemented!()
    }

    fn market_depth(&self, pair: &str) -> CoreResult<Depth> {
        unimplemented!()
    }

    fn symbol_and_base_to_pair_format(&self, symbol: &str, base: &str) -> String {
        unimplemented!()
    }

    fn stop_loss(
        &self,
        symbol: &str,
        amount: f64,
        stop_price: f64,
        limit_price: f64,
    ) -> CoreResult<()> {
        unimplemented!()
    }
}

fn split_symbol_and_base(pair: &str) -> Option<(String, String)> {
    for base in BASE_PAIRS.iter() {
        if pair.ends_with(base) {
            return Some((
                pair.trim_end_matches(base).to_string().to_uppercase(),
                base.to_string().to_uppercase(),
            ));
        };
    }
    None
}

fn string_to_pair(pair: &str) -> Option<Pair> {
    split_symbol_and_base(&pair).map(|(symbol, base)| Pair { base, symbol })
}

fn pair_to_string(pair: Pair) -> String {
    format!("{}{}", pair.symbol.to_lowercase(), pair.base.to_lowercase())
}

fn string_to_trade_type(order_type: &str) -> OrderType {
    match order_type {
        "buy-market" | "sell-limit" => OrderType::Limit,
        "sell-market" | "buy-market" => OrderType::Market,
        _ => OrderType::Limit,
    }
}

fn string_to_order_type(order_type: &str) -> TradeType {
    match order_type {
        "buy-market" | "sell-market" => TradeType::Buy,
        "sell-limit" | "buy-market" => TradeType::Sell,
        _ => TradeType::Buy,
    }
}
