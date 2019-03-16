pub mod config;
pub mod error;
pub mod exchanges;
pub mod threadpool;
pub(crate) mod utils;

pub mod models {
    mod asset;
    pub use self::asset::*;
    mod candlestick;
    pub use self::candlestick::*;
    mod order;
    pub use self::order::*;
    mod trade;
    pub use self::trade::*;
    mod trade_type;
    pub use self::trade_type::*;
    mod position;
    pub use self::position::*;
    mod pair;
    pub use self::pair::*;
    mod book_ticker;
    pub use self::book_ticker::*;
    mod depth;
    pub use self::depth::*;
}

pub mod presenters {
    mod asset;
    mod balance;
    mod order;
    mod position;
    mod trade_presenter;

    pub use self::{asset::*, balance::*, order::*, position::*, trade_presenter::*};
}

pub mod indicators {
    pub mod bbands;
    pub mod macd;
    pub mod ote;
    pub mod rsi;
    pub mod sma;

    pub use self::{bbands::*, macd::*, ote::*, rsi::*, sma::*};
}

pub mod socket {
    pub mod binance;
    pub use self::binance::BinanceWS;

    #[derive(Debug)]
    pub enum Event {
        PriceChange(String, f64, f64),
    }
}

pub static KNOWN_STABLECOIN_SYMBOLS: [&str; 3] = ["USDT", "USD", "TUSD"];
pub static KNOWN_BTC_SYMBOLS: [&str; 2] = ["XBT", "BTC"];
