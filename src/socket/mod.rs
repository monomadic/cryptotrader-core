pub mod binance;
pub use self::binance::BinanceWS;

#[derive(Debug)]
pub enum Event {
    PriceChange(String, f64, f64),
}
