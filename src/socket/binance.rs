#![allow(dead_code)]
#![allow(unused_variables)]

// NOTE: Binance rate limits are: 1200 requests per minute; 10 orders per second; 100,000 orders per 24hrs. 

use binance_api::websockets::MarketEventHandler;
use binance_api::model::TradesEvent;
use binance_api::model::DepthOrderBookEvent;
use binance_api::model::OrderBook;
use binance_api::websockets::WebSockets;

use crate::error::TrailerError;
// use ::models::*;
use super::Event;

pub struct BinanceWS {
    event_callback: Box<Fn(Event)>,
}

struct BinanceWebSocketHandler;

impl BinanceWS {
    pub fn new(event_callback: impl Fn(Event) + 'static) -> Self {
        BinanceWS {
            event_callback: Box::new(event_callback),
        }
    }

    pub fn run(self) -> Result<(), TrailerError> {
        let mut socket: WebSockets = WebSockets::new();
        // let agg_trade: String = format!("{}@aggTrade", "icxbtc");

        socket.add_market_handler(self);
        socket.connect("icxbtc@aggTrade").unwrap(); // check error
        socket.event_loop();

        Ok(())
    }
}

impl MarketEventHandler for BinanceWS {
    fn aggregated_trades_handler(&self, event: &TradesEvent) {
        let e = Event::PriceChange(
            event.clone().symbol,
            event.price.parse::<f64>().unwrap(),
            event.qty.parse::<f64>().unwrap(),
        );

        (self.event_callback)(e);
    }
    fn depth_orderbook_handler(&self, model: &DepthOrderBookEvent) { println!("{:?}", model); }
    fn partial_orderbook_handler(&self, model: &OrderBook) { println!("{:?}", model); }
}
