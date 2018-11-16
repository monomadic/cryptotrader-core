use ::models::*;
use ::error::*;
use ::exchanges::{ ExchangeAPI, Exchange };

pub struct BitfinexAPI {
}

impl ExchangeAPI for BitfinexAPI {
    fn display(&self) -> String { "Bitfinex".to_string() }
    fn btc_price(&self) -> Result<f64, TrailerError> { Err(TrailerError::unsupported()) }
    fn funds(&self) -> Result<Funds, TrailerError> { Err(TrailerError::unsupported()) }
    fn balances(&self) -> Result<Vec<Asset>, TrailerError> { Err(TrailerError::unsupported()) }
    fn price(&self, symbol: &str) -> Result<f64, TrailerError> { Err(TrailerError::unsupported()) }
    fn prices(&self) -> Result<Prices, TrailerError> { Err(TrailerError::unsupported()) }
    fn limit_buy(&self, symbol: &str, amount: f64, price: f64) -> Result<(), TrailerError> { Err(TrailerError::unsupported()) }
    fn limit_sell(&self, symbol: &str, amount: f64, price: f64) -> Result<(), TrailerError> { Err(TrailerError::unsupported()) }
    fn open_orders(&self) -> Result<Vec<Order>, TrailerError> { Err(TrailerError::unsupported()) }
    fn past_orders(&self) -> Result<Vec<Order>, TrailerError> { Err(TrailerError::unsupported()) }
    fn past_trades_for(&self, symbol: &str) -> Result<Vec<Order>, TrailerError> { Err(TrailerError::unsupported()) }
    fn chart_data(&self, symbol: &str, interval: &str) -> Result<Vec<Candlestick>, TrailerError> { Err(TrailerError::unsupported()) }
}

pub fn connect(api_key: &str, secret_key: &str) -> BitfinexAPI {
    BitfinexAPI {
        account: Binance::new(
            Some(api_key.to_string()),
            Some(secret_key.to_string())
        ),
        market: Market::new(None, None),
    }
}