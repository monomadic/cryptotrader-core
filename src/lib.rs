pub mod config;
pub mod error;
pub mod exchanges;
pub mod threadpool;

pub mod models {
    mod asset; pub use self::asset::*;
    mod funds; pub use self::funds::*;
    mod candlestick; pub use self::candlestick::*;
    mod order; pub use self::order::*;
    mod trades; pub use self::trades::*;
    mod trade_type; pub use self::trade_type::*;
    mod position; pub use self::position::*;
    mod price; pub use self::price::*;
    mod balance; pub use self::balance::*;
}

pub mod presenters {
    mod asset;
    mod funds;
    mod position;
    mod order;

    pub use self::{ asset::*, funds::*, position::*, order::* };
}

pub mod indicators {
    pub mod rsi;
}

pub mod socket {
    pub mod binance; pub use self::binance::BinanceWS;

    #[derive(Debug)]
    pub enum Event {
        PriceChange(String, f64, f64),
    }
}