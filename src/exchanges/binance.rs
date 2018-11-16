#![allow(dead_code)]
#![allow(unused_variables)]

use binance::api::*;
use binance::account::*;
use binance::market::*;
use binance::websockets::*;

use std::collections::HashMap;

use ::models::*;
use ::error::*;
use ::exchanges::*;

#[derive(Clone)]
pub struct BinanceAPI {
    account: Account,
    market: Market,
}

pub struct BinanceWS {
    socket: WebSockets,
}

use binance::model::{ TradesEvent, DepthOrderBookEvent, OrderBook };

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

impl ExchangeAPI for BinanceAPI {
    fn display(&self) -> String { "Binance".to_string() }
    fn btc_symbol(&self) -> String { "BTC".into() }
    fn usd_symbol(&self) -> String { "USDT".into() }

    fn btc_price(&self) -> Result<f64, TrailerError> { Ok(self.price("BTCUSDT").or(Err(TrailerError::generic("could not find the BTCUSDT symbol.")))?) }

    // basic (unenriched) list of
    // fn balances(&self) -> Result<Balances, TrailerError> {
    //     let balances = self.balances()?;

    //     Ok(balances)
    // }


    fn funds(&self) -> Result<Funds, TrailerError> {
        let balances = self.balances()?;
        // let prices = self.prices()?;
        let btc = balances.clone().into_iter().find(|c| c.symbol == "BTC");
        let usdt = balances.clone().into_iter().find(|c| c.symbol == "USDT");

        let alts_all:Vec<Asset> = balances.clone().into_iter().filter(|c| c.symbol != "USDT" && c.symbol != "BTC").collect();
        let alts:Vec<Asset> = alts_all.into_iter().filter(|c| c.amount > 0.9).collect();

        // let btc_price = self.btc_price()?;

        // // assign a price to btc if it exists.
        // if let Some(ref mut b) = btc {
        //     b.value_in_btc = Some(1.0);
        //     b.value_in_usd = Some(btc_price);
        // }
        
        // add prices
        // for mut alt in alts.iter_mut() {
        //     alt.value_in_btc = prices.get(&format!("{}{}", alt.symbol, "BTC")).cloned();
        // }

        Ok(Funds {
            btc:                btc,
            fiat:               balances.clone().into_iter().filter(|c| c.symbol == "USDT").collect(),
            alts:               alts,
        })
    }

    /// Simple list of balances
    fn balances(&self) -> Result<Vec<Asset>, TrailerError> {
        let result = self.account.get_account()?;

        Ok(result.balances.into_iter().map(|balance| {
            Asset {
                symbol: balance.asset,
                amount: balance.free.parse::<f64>().unwrap() + balance.locked.parse::<f64>().unwrap(),
                locked: balance.locked.parse::<f64>().unwrap(),
                exchange: Exchange::Binance,
                // value_in_btc: None,
                // value_in_usd: None,
            }
        }).filter(|b| b.amount > 0.0).collect())
    }

    fn price(&self, symbol: &str) -> Result<f64, TrailerError> {
        Ok(self.market.get_price(symbol)?)
    }

    fn prices(&self) -> Result<::models::Prices, TrailerError> {
        let market_prices = self.market.get_all_prices()?;
        let mut p = HashMap::new();

        match market_prices {
            ::binance::model::Prices::AllPrices(prices) => {
                for price in prices {
                    p.insert(
                        price.symbol,
                        price.price);
                }
            }
        }

        Ok(p)
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

    fn stop_loss(&self, symbol: &str, amount: f64, stop_price: f64, limit_price: f64) -> Result<(), TrailerError> {
        Err(TrailerError::unsupported())
    }

    fn open_orders(&self) -> Result<Vec<Order>, TrailerError> {
        Ok(self.account.get_open_orders_all()?.into_iter().map(|order| {
            Order{
                id:             order.order_id.to_string(),
                symbol:         order.symbol,
                order_type:     TradeType::is_buy(order.side == "Buy"),
                qty:            order.executed_qty.parse::<f64>().unwrap(),
                price:          order.orig_qty.parse::<f64>().unwrap(),
            }
        }).collect())
    }

    fn past_orders(&self) -> Result<Vec<Order>, TrailerError> {
        Err(TrailerError::unsupported())
    }

    fn trades_for(&self, symbol: &str) -> Result<Vec<Order>, TrailerError> {
        Ok(self.account.trade_history(symbol)?.into_iter().map(|order| {
            Order{
                id:             order.id.to_string(),
                symbol:         symbol.to_string(),
                order_type:     TradeType::is_buy(order.is_buyer),
                qty:            order.qty,
                price:          order.price,
            }
        }).collect())
    }

    fn chart_data(&self, symbol: &str, interval: &str) -> Result<Vec<Candlestick>, TrailerError> {
        Ok(self.market.get_klines(symbol, interval)?.iter().map(|cs|
            Candlestick {
                open_time:          cs.open_time as i32,
                open_price:         cs.open_price,
                close_price:        cs.close_price,
                high_price:         cs.high_price,
                low_price:          cs.low_price,
                volume:             cs.volume,
                number_of_trades:   cs.trades as u64,
            }
        ).collect())
    }
}

use binance::errors::Error as BinanceError;
impl From<BinanceError> for ::error::TrailerError {
    fn from(error: BinanceError) -> Self {
        ::error::TrailerError {
            error_type: ::error::TrailerErrorType::APIError,
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
        account: Binance::new(
            Some(api_key.to_string()),
            Some(secret_key.to_string())
        ),
        market: Market::new(None, None),
    }
}

impl BinanceAPI {

    pub fn trades(&self, coin: &str) -> Vec<Trade> {
        match self.account.trade_history(coin) {
            Ok(answer) => {
                answer.iter().map(|trade| {
                    let cost = trade.price;
                    let qty = trade.qty;

                    Trade { cost: cost, qty: qty, buy: trade.is_buyer }
                }).collect()
            },
            Err(e) => {
                println!("Error: {}", e);
                Vec::new()
            },
        }
    }

    // pub fn show_trades(&self, coin: &str) {

    //     match self.account.trade_history(coin.into()) {
    //         Ok(answer) => {
    //             println!("\nTrade History: {}", coin);
    //             let mut total_cost = 0.0_f64;
    //             let mut total_amount = 0.0f64;
    //             // let average_buy_price = 0.0_f64;

    //             let mut total_buy_cost = 0.0_f64;
    //             let mut total_buy_amount = 0.0_f64;

    //             let mut total_sell_cost = 0.0_f64;
    //             let mut total_sell_amount = 0.0_f64;

    //             for trade in answer {
    //                 let cost = trade.price.parse::<f64>().unwrap();
    //                 let qty = trade.qty.parse::<f64>().unwrap();
    //                 // println!("{:?}", trade);
    //                 if trade.is_buyer {
    //                     total_amount = total_amount + qty;
    //                     total_cost = total_cost + cost;

    //                     total_buy_cost = total_buy_cost + (qty * cost);
    //                     total_buy_amount = total_buy_amount + qty;
    //                     println!("+ {:12} {:12} b: {:.2}", trade.qty.green(), trade.price, total_amount);
    //                 } else {
    //                     total_amount = total_amount - qty;
    //                     total_cost = total_cost - cost;

    //                     total_sell_cost = total_sell_cost + (qty * cost);
    //                     total_sell_amount = total_sell_amount + qty;

    //                     println!("- {:12} {:12} b: {:.2}", trade.qty.red(), trade.price, total_amount);
    //                 }
    //             }

    //             println!("\n{} average buy cost:\n\tall time: {}", coin, format!("{:.8}", total_buy_cost / total_buy_amount).green());
    //             println!("\n{} average sell cost:\n\tall time: {}", coin, format!("{:.8}", total_sell_cost / total_sell_amount).red());
    //         },
    //         Err(e) => println!("Error: {}", e),
    //     }
    // }

}
