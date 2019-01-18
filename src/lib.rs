extern crate binance;
// extern crate bittrex_api as bittrex;
extern crate reqwest;
// extern crate kucoin;
// extern crate bitfinex;
extern crate math;

extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
// extern crate ratelimit;
// extern crate itertools;
extern crate ta_lib_wrapper as talib;
extern crate rayon;
extern crate dirs;

pub mod config;
pub mod error;
pub mod exchanges;
pub mod models;
pub mod presenters;
pub mod socket;
pub mod indicators;
pub mod threadpool;
