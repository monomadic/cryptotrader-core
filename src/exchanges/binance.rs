#![allow(dead_code)]
#![allow(unused_variables)]

use crate::utils::*;
use crate::{error::*, exchanges::*, models::*};
use log::info;

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

pub static BASE_PAIRS: [&str; 10] = [
    "USDT", "BTC", "ETH", "USDC", "TUSD", "BNB", "USDS", "BNB", "PAX", "XRP",
];
// pub static BASE_PAIRS: [&str; 2] = ["USDT", "BTC"];
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

    fn base_pairs(&self) -> Vec<String> {
        BASE_PAIRS
            .into_iter()
            .map(|pair| pair.to_string())
            .collect()
    }

    fn market_depth(&self, pair: &str) -> CoreResult<Depth> {
        let depth = self.market.get_depth(pair)?;
        // println!("{:?}", depth);

        Ok(Depth {
            bids: depth
                .bids
                .into_iter()
                .map(|o| Offer {
                    price: o.price,
                    qty: o.qty,
                })
                .collect(),
            asks: depth
                .asks
                .into_iter()
                .map(|o| Offer {
                    price: o.price,
                    qty: o.qty,
                })
                .collect(),
        })
    }

    /// Simple list of balances
    fn balances(&self) -> CoreResult<Vec<Asset>> {
        let result = self.account.get_account()?;
        let assets: Vec<Asset> = result
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
            .collect();

        info!(
            "response: found assets: {}",
            assets
                .clone()
                .into_iter()
                .map(|p| format!("[{} - {}]", p.symbol, p.amount))
                .collect::<Vec<String>>()
                .join(", ")
        );

        Ok(assets)
    }

    fn pair(&self, pair: &str) -> CoreResult<Pair> {
        let price = self.market.get_price(pair)?;
        let (symbol, base) =
            split_symbol_and_base(pair).ok_or(TrailerError::PairNotFound(pair.to_string()))?;

        Ok(Pair {
            symbol: symbol.to_string(),
            base: base.to_string(),
        })
    }

    fn all_pairs(&self) -> CoreResult<Vec<Pair>> {
        let binance_api::model::Prices::AllPrices(prices) = self.market.get_all_prices()?;

        Ok(prices
            .into_iter()
            .map(|pair| string_to_pair(&pair.symbol))
            .filter(|r| r.is_some())
            .map(|r| r.unwrap())
            .collect())
    }

    fn all_prices(&self) -> CoreResult<Vec<Price>> {
        let binance_api::model::Prices::AllPrices(prices) = self.market.get_all_prices()?;

        Ok(prices
            .into_iter()
            .map(|price| {
                string_to_pair(&price.symbol).map(|pair| Price {
                    pair: pair,
                    price: price.price,
                })
            })
            .filter(|r| r.is_some())
            .map(|r| r.unwrap())
            .collect())
    }

    fn symbol_and_base_to_pair_format(&self, symbol: &str, base: &str) -> String {
        format!("{}{}", symbol, base)
    }

    fn book_tickers(&self) -> CoreResult<Vec<BookTicker>> {
        let binance_api::model::BookTickers::AllBookTickers(tickers) =
            self.market.get_all_book_tickers()?;

        Ok(tickers
            .into_iter()
            .map(|ticker| {
                let (symbol, base) = split_symbol_and_base(&ticker.symbol)
                    .expect(&format!("could not split symbol: {:?}", &ticker.symbol));

                BookTicker {
                    pair: Pair {
                        symbol: symbol,
                        base: base,
                    },
                    bid_price: ticker.bid_price,
                    bid_qty: ticker.bid_qty,
                    ask_price: ticker.ask_price,
                    ask_qty: ticker.ask_qty,
                }
            })
            .collect())
    }

    fn limit_buy(&self, symbol: &str, amount: f64, price: f64) -> CoreResult<()> {
        let result = self.account.limit_buy(symbol, amount, price)?;
        println!("{:?}", result);
        Ok(())
    }

    fn limit_sell(&self, symbol: &str, amount: f64, price: f64) -> CoreResult<()> {
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
    ) -> CoreResult<()> {
        Err(Box::new(TrailerError::Unsupported))
    }

    fn open_orders(&self) -> CoreResult<Vec<Order>> {
        fn parse_order_type(order_type: &str) -> OrderType {
            match order_type {
                "LIMIT" => OrderType::Limit,
                "MARKET" => OrderType::Market,
                "STOP_LOSS" => OrderType::StopLoss,
                "STOP_LOSS_LIMIT" => OrderType::StopLossLimit,
                "TAKE_PROFIT" => OrderType::TakeProfit,
                "TAKE_PROFIT_LIMIT" => OrderType::TakeProfitLimit,
                "LIMIT_MAKER" => OrderType::LimitMaker,
                _ => OrderType::Limit, // TODO: throw Err here...
            }
        }

        fn parse_trade_type(trade_type: &str) -> TradeType {
            match trade_type {
                "BUY" => TradeType::Buy,
                "SELL" => TradeType::Sell,
                _ => TradeType::Sell, // TODO: throw Err here...
            }
        }

        let mut results = Vec::new();

        for order in self.account.get_open_orders_all()? {
            // let pair = self
            //     .string_to_pair(order.symbol.to_string(), 0.0)
            //     .ok_or(TrailerError::PairNotFound(order.symbol.clone()))?;
            let pair = string_to_pair(&order.symbol)
                .ok_or(TrailerError::PairNotFound(order.symbol.to_string()))?;

            let (symbol, base) = split_symbol_and_base(&order.symbol)
                .ok_or(TrailerError::PairNotFound(order.symbol.clone()))?;

            // let pair = find_pair_by_symbol_and_base(&symbol, &base, PRICES.clone())
            // .ok_or(TrailerError::PairNotFound(order.symbol.clone()))?;

            results.push(Order {
                id: order.order_id.to_string(),
                pair,
                order_type: parse_order_type(&order.type_name),
                trade_type: parse_trade_type(&order.side),
                purchase_price: order.price,
                qty: order.orig_qty.parse::<f64>().unwrap(),
                executed_qty: order.executed_qty.parse::<f64>().unwrap(),
                time: local_datetime_from_unix(order.time),
            });
        }

        Ok(results)
    }

    fn past_orders(&self) -> CoreResult<Vec<Order>> {
        Err(Box::new(TrailerError::Unsupported))
    }

    // fn trades_for(&self, symbol: &str) -> CoreResult<Vec<Trade>> {
    //     info!("BINANCE: trades_for({})", symbol);
    //     // self.trades_for_pair(symbol);
    //     panic!("unsupported");
    // }

    /// find all trades for a single trading pair (market).
    fn trades_for_pair(&self, pair: Pair) -> CoreResult<Vec<Trade>> {
        info!("BINANCE: trades_for_pair({})", pair.clone());

        let result = self.account.trade_history(pair_to_string(pair.clone()))?;
        info!("result: {:?}", result);

        let mut trades: Vec<Trade> = result
            .into_iter()
            .map(|trade| Trade {
                id: trade.id.to_string(),
                time: local_datetime_from_unix(trade.time),
                pair: pair.clone(),
                trade_type: TradeType::is_buy(trade.is_buyer),
                qty: trade.qty,
                sale_price: trade.price,
                fee: trade.commission.parse::<f64>().unwrap_or(0.0),
                fee_symbol: Some(trade.commission_asset),
            })
            .collect();

        // sort by time
        trades.sort_by(|a, b| a.time.cmp(&b.time));

        Ok(trades)
    }

    /// find all trades for a symbol across all base pairs.
    fn trades_for_symbol(&self, symbol: &str, pairs: Vec<Pair>) -> CoreResult<Vec<Trade>> {
        info!("BINANCE: trades_for_symbol({})", symbol);

        let symbol = symbol.to_uppercase();
        let pairs = Pair::base_pairs_for_symbol(&symbol, &pairs);

        let mut trades = Vec::new();

        for pair in pairs {
            let mut result = self.trades_for_pair(pair)?;
            info!("result: {:?}", result);
            trades.append(&mut result);
        }

        // sort by time
        trades.sort_by(|a, b| a.time.cmp(&b.time));

        Ok(trades)
    }

    // fn trades_for(&self, symbol: &str) -> CoreResult<Vec<Order>> {
    //     Ok(self
    //         .account
    //         .trade_history(symbol)?
    //         .into_iter()
    //         .map(|order| Order {
    //             id: order.id.to_string(),
    //             symbol: symbol.to_string(),
    //             order_type: TradeType::is_buy(order.is_buyer),
    //             qty: order.qty,
    //             price: order.price,
    //         })
    //         .collect())
    // }

    fn chart_data(&self, pair: &str, interval: &str) -> CoreResult<Vec<Candlestick>> {
        Ok(self
            .market
            .get_klines(pair, interval)?
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

fn split_symbol_and_base(pair: &str) -> Option<(String, String)> {
    for base in BASE_PAIRS.iter() {
        if pair.ends_with(base) {
            return Some((pair.trim_end_matches(base).to_string(), base.to_string()));
        };
    }
    None
}

fn string_to_pair(pair: &str) -> Option<Pair> {
    split_symbol_and_base(&pair).map(|(symbol, base)| Pair { base, symbol })
}

fn pair_to_string(pair: Pair) -> String {
    format!("{}{}", pair.symbol, pair.base)
}
