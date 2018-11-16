#![allow(dead_code)]
#![allow(unused_variables)]

use bittrex::BittrexClient;

use std::collections::HashMap;

use ::models::*;
use ::error::*;
use ::exchanges::*;

pub struct BittrexAPI {
    client: BittrexClient,
}

use bittrex::error::BittrexError as BittrexError;
impl From<BittrexError> for TrailerError {
    fn from(error: BittrexError) -> Self {
        TrailerError {
            error_type: TrailerErrorType::APIError,
            message: error.message,
        }
    }
}

pub fn connect(api_key: &str, secret_key: &str) -> BittrexAPI {
    BittrexAPI {
        client: BittrexClient::new(
            api_key.to_string(),
            secret_key.to_string()
        ),
    }
}

impl ExchangeAPI for BittrexAPI {
    
    fn display(&self) -> String { "Bittrex".to_string() }
    fn btc_symbol(&self) -> String { "BTC".into() }
    fn usd_symbol(&self) -> String { "USDT".into() }

    fn btc_price(&self) -> Result<f64, TrailerError> { Ok(self.price("USD-BTC")?) }

    fn funds(&self) -> Result<Funds, TrailerError> {
        let balances = self.balances()?;
        let prices = self.prices()?;

        let alts_all:Vec<Asset> = balances.clone().into_iter().filter(|c| c.symbol != "USDT" && c.symbol != "BTC").collect();
        let alts:Vec<Asset> = alts_all.into_iter().filter(|c| c.amount > 0.9).collect();

        Ok(Funds {
            btc:    balances.clone().into_iter().find(|c| c.symbol == "BTC"),
            fiat:   balances.clone().into_iter().filter(|c| c.symbol == "USDT").collect(),
            alts:   alts,
        })
    }

    fn balances(&self) -> Result<Vec<Asset>, TrailerError> {
        let balances = self.client.get_balances()?;

        Ok(balances.into_iter().map(|balance| {
            Asset {
                symbol: balance.currency,
                amount: balance.balance as f64,
                locked: (balance.balance - balance.available) as f64,
                exchange: Exchange::Bittrex,
            }
        }).collect())
    }

    fn price(&self, symbol: &str) -> Result<f64, TrailerError> {
        Ok(self.client.get_ticker(symbol)?.last as f64)
    }

    fn prices(&self) -> Result<Prices, TrailerError> {
        let response = self.client.get_market_summaries()?;
        let mut p = HashMap::new();

        for market in response {
            let split: Vec<&str> = market.market_name.split("-").collect();
            let pair_name = format!("{}{}", *split.last().unwrap(), *split.first().unwrap()); // dangerous, fix

            p.insert(
                pair_name,
                market.last
            );
        }
        
        Ok(p)
    }

    fn limit_buy(&self, symbol: &str, amount: f64, price: f64) -> Result<(), TrailerError> {
        let result = self.client.buy_limit(symbol, amount, price)?;
        println!("{}", result);
        Ok(())
    }

    fn limit_sell(&self, symbol: &str, amount: f64, price: f64) -> Result<(), TrailerError> {
        let result = self.client.sell_limit(symbol, amount, price)?;
        println!("{}", result);
        Ok(())
    }

    fn stop_loss(&self, symbol: &str, amount: f64, stop_price: f64, limit_price: f64) -> Result<(), TrailerError> {
        Err(TrailerError::unsupported())
    }

    fn open_orders(&self) -> Result<Vec<Order>, TrailerError> {
        Ok(self.client.get_open_orders()?.into_iter().map(|order| {
            Order{
                id:             order.order_uuid,
                symbol:         order.exchange,
                order_type:     TradeType::is_buy(order.order_type == "buy"),
                qty:            order.quantity as f64,
                price:          order.limit as f64,
            }
        }).collect())
    }

    fn past_orders(&self) -> Result<Vec<Order>, TrailerError> {
        Ok(self.client.get_order_history()?.into_iter().map(|order| {
            {
                Order{
                    id:             order.order_uuid,
                    symbol:         order.exchange,
                    order_type:     TradeType::is_buy(order.order_type == "buy"),
                    qty:            order.quantity as f64,
                    price:          order.limit as f64,
                }
            }
        }).collect())
    }

    fn past_trades_for(&self, symbol: &str) -> Result<Vec<Order>, TrailerError> {
        Err(TrailerError::unsupported())
    }

    fn chart_data(&self, symbol: &str, interval: &str) -> Result<Vec<Candlestick>, TrailerError> {
        Err(TrailerError::unsupported())
    }
}
