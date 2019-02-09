pub mod config;
pub mod error;
pub mod exchanges;
pub mod threadpool;
pub(crate) mod utils;

pub mod models {
    mod asset; pub use self::asset::*;
    mod candlestick; pub use self::candlestick::*;
    mod order; pub use self::order::*;
    mod trades; pub use self::trades::*;
    mod trade_type; pub use self::trade_type::*;
    mod position; pub use self::position::*;
    mod pair; pub use self::pair::*;
}

pub mod presenters {
    mod asset;
    mod position;
    mod order;

    pub use self::{ asset::*, position::*, order::* };
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

pub static KNOWN_STABLECOIN_SYMBOLS: [&str; 3] = ["USDT", "USD", "TUSD"];
pub static KNOWN_BTC_SYMBOLS: [&str; 2] = ["XBT", "BTC"];
