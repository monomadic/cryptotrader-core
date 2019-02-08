#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{error::*, exchanges::*, models::*};

use binance_api::{
    account::*,
    api::*,
    market::*,
    model::{DepthOrderBookEvent, OrderBook, TradesEvent},
    websockets::*,
};

#[derive(Clone)]
pub struct BinanceAPI {
    account: Account,
    market: Market,
}

pub struct BinanceWS {
    socket: WebSockets,
}

// pub static BASE_PAIRS: [&str; 7] = ["USDT", "BTC", "ETH", "USDC", "TUSD", "BNB", "USDS"];
pub static BASE_PAIRS: [&str; 2] = ["USDT", "BTC"];
pub static BTC_SYMBOL: &str = "BTC";
pub static USD_SYMBOL: &str = "USDT";

struct BinanceWebSocketHandler;

impl MarketEventHandler for BinanceWebSocketHandler {
    fn aggregated_trades_handler(&self, event: &TradesEvent) {
        println!(
            "- Symbol: {}, price: {}, qty: {}",
            event.symbol, event.price, event.qty
        );
    }
    fn depth_orderbook_handler(&self, model: &DepthOrderBookEvent) {}
    fn partial_orderbook_handler(&self, model: &OrderBook) {}
}

impl BinanceAPI {
    pub fn connect(api_key: &str, secret_key: &str) -> Self {
        Self {
            account: Binance::new(Some(api_key.to_string()), Some(secret_key.to_string())),
            market: Market::new(None, None),
        }
    }
}

impl ExchangeAPI for BinanceAPI {
    fn display(&self) -> String {
        "Binance".to_string()
    }
    fn btc_symbol(&self) -> String {
        BTC_SYMBOL.into()
    }
    fn usd_symbol(&self) -> String {
        USD_SYMBOL.into()
    }
    fn btc_price(&self) -> Result<Pair, TrailerError> {
        self.pair("BTCUSDT")
    }
    fn base_pairs(&self) -> Vec<String> {
        BASE_PAIRS
            .into_iter()
            .map(|pair| pair.to_string())
            .collect()
    }

    /// Simple list of balances
    fn balances(&self) -> Result<Vec<Asset>, TrailerError> {
        let result = self.account.get_account()?;

        Ok(result
            .balances
            .into_iter()
            .map(|balance| Asset {
                symbol: balance.asset,
                amount: balance.free.parse::<f64>().unwrap()
                    + balance.locked.parse::<f64>().unwrap(),
                locked: balance.locked.parse::<f64>().unwrap(),
                exchange: Exchange::Binance,
            })
            .filter(|b| b.amount > 0.0)
            .collect())
    }

    fn pair(&self, pair: &str) -> Result<Pair, TrailerError> {
        let price = self.market.get_price(pair)?;
        let (symbol, base) = split_symbol_and_base(pair)?;

        Ok(Pair {
            symbol: symbol.to_string(),
            base: base.to_string(),
            price,
        })
    }

    fn all_pairs(&self) -> Result<Vec<Pair>, TrailerError> {
        let binance_api::model::Prices::AllPrices(prices) = self.market.get_all_prices()?;

        Ok(prices
            .into_iter()
            .map(|pair| {
                let price = pair.price;
                match split_symbol_and_base(&pair.symbol) {
                    Ok((symbol, base)) => Some(Pair {
                        base,
                        price,
                        symbol,
                    }),
                    Err(e) => None,
                }
            })
            .filter(|r| r.is_some())
            .map(|r| r.unwrap())
            .collect())
    }

    fn pair_format(&self, pair: Pair) -> String {
        format!("{}{}", pair.symbol, pair.base)
    }

    fn limit_buy(&self, symbol: &str, amount: f64, price: f64) -> Result<(), TrailerError> {
        let result = self.account.limit_buy(symbol, amount, price)?;
        println!("{:?}", result);
        Ok(())
    }

    fn limit_sell(&self, symbol: &str, amount: f64, price: f64) -> Result<(), TrailerError> {
        let result = self.account.limit_sell(symbol, amount, price)?;
        println!("{:?}", result);
        Ok(())
    }

    fn stop_loss(
        &self,
        symbol: &str,
        amount: f64,
        stop_price: f64,
        limit_price: f64,
    ) -> Result<(), TrailerError> {
        Err(TrailerError::unsupported())
    }

    fn open_orders(&self) -> Result<Vec<Order>, TrailerError> {
        Ok(self
            .account
            .get_open_orders_all()?
            .into_iter()
            .map(|order| Order {
                id: order.order_id.to_string(),
                symbol: order.symbol,
                order_type: TradeType::is_buy(order.side == "Buy"),
                qty: order.executed_qty.parse::<f64>().unwrap(),
                price: order.orig_qty.parse::<f64>().unwrap(),
            })
            .collect())
    }

    fn past_orders(&self) -> Result<Vec<Order>, TrailerError> {
        Err(TrailerError::unsupported())
    }

    fn trades_for(&self, symbol: &str) -> Result<Vec<Order>, TrailerError> {
        Ok(self
            .account
            .trade_history(symbol)?
            .into_iter()
            .map(|order| Order {
                id: order.id.to_string(),
                symbol: symbol.to_string(),
                order_type: TradeType::is_buy(order.is_buyer),
                qty: order.qty,
                price: order.price,
            })
            .collect())
    }

    fn chart_data(&self, symbol: &str, interval: &str) -> Result<Vec<Candlestick>, TrailerError> {
        Ok(self
            .market
            .get_klines(symbol, interval)?
            .iter()
            .map(|cs| Candlestick {
                open_time: cs.open_time as i32,
                open_price: cs.open_price,
                close_price: cs.close_price,
                high_price: cs.high_price,
                low_price: cs.low_price,
                volume: cs.volume,
                number_of_trades: cs.trades as u64,
            })
            .collect())
    }
}

use binance_api::errors::Error as BinanceError;
impl From<BinanceError> for crate::error::TrailerError {
    fn from(error: BinanceError) -> Self {
        crate::error::TrailerError {
            error_type: crate::error::TrailerErrorType::APIError(error.description().to_string()),
            message: error.description().to_string(),
        }
    }
}

pub fn ws(symbol: String) {
    println!("attempting to connect to binance with symbol: {}", symbol);
    let agg_trade: String = format!("{}@aggTrade", symbol);
    let mut web_socket: WebSockets = WebSockets::new();

    web_socket.add_market_handler(BinanceWebSocketHandler);
    web_socket.connect(&agg_trade).unwrap(); // check error
    web_socket.event_loop();
}

pub fn connect(api_key: &str, secret_key: &str) -> BinanceAPI {
    BinanceAPI {
        account: Binance::new(Some(api_key.to_string()), Some(secret_key.to_string())),
        market: Market::new(None, None),
    }
}

impl BinanceAPI {
    pub fn trades(&self, coin: &str) -> Vec<Trade> {
        match self.account.trade_history(coin) {
            Ok(answer) => answer
                .iter()
                .map(|trade| {
                    let cost = trade.price;
                    let qty = trade.qty;

                    Trade {
                        cost,
                        qty,
                        buy: trade.is_buyer,
                    }
                })
                .collect(),
            Err(e) => {
                println!("Error: {}", e);
                Vec::new()
            }
        }
    }
}

fn split_symbol_and_base(pair: &str) -> Result<(String, String), TrailerError> {
    for base in BASE_PAIRS.iter() {
        if pair.ends_with(base) {
            return Ok((pair.trim_end_matches(base).to_string(), base.to_string()));
        };
    }
    Err(TrailerError::generic("base pair not found"))
}
